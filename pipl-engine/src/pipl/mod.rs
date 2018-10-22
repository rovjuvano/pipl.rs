pub mod mods;

use ::Name;
use ::Mods;
use ::Molecule;
use ::ReadMolecule;
use ::Refs;
use ::SendMolecule;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
pub struct Pipl<T> {
    map: ReactionMap<T>,
}
impl<T> Pipl<T> {
    pub fn new() -> Self {
        Pipl {
            map: ReactionMap::new(),
        }
    }
    pub fn add(&mut self, molecule: Molecule<T>) {
        self.add_molecule(molecule, Refs::new());
    }
    pub fn read(&mut self, read: ReadMolecule<T>) {
        self.add_read(read, Refs::new());
    }
    pub fn send(&mut self, send: SendMolecule<T>) {
        self.add_send(send, Refs::new());
    }
    fn add_molecule(&mut self, molecule: Molecule<T>, refs: Refs<T>) {
        match molecule {
            Molecule::Read(read) => self.add_read(read, refs),
            Molecule::Send(send) => self.add_send(send, refs),
        }
    }
    fn add_read(&mut self, read: ReadMolecule<T>, refs: Refs<T>) {
        self.map.add_read(ReadReaction::new(read, refs));
    }
    fn add_send(&mut self, send: SendMolecule<T>, refs: Refs<T>) {
        self.map.add_send(SendReaction::new(send, refs));
    }
    fn add_choice(&mut self, molecules: Vec<Molecule<T>>, refs: Refs<T>) {
        self.map.add_choice(ChoiceReaction::new(molecules, refs));
    }
    pub fn step(&mut self) {
        if let Some((reader, sender)) = self.map.next() {
            let ReadReaction { read, refs: read_refs } = reader;
            let SendReaction { send, refs: send_refs } = sender;
            let mut mods = Mods::new();
            let names = send.send(&mut mods, send_refs);
            read.read(&mut mods, read_refs, names);
            mods.apply(self);
        }
    }
}
#[derive(Debug)]
enum Reaction<T> {
    Choice(Rc<ChoiceReaction<T>>),
    Read(ReadReaction<T>),
    Send(SendReaction<T>),
}
#[derive(Debug)]
struct ReadReaction<T> {
    read: ReadMolecule<T>,
    refs: Refs<T>,
}
impl<T> ReadReaction<T> {
    fn new(read: ReadMolecule<T>, refs: Refs<T>) -> Self {
        ReadReaction { read, refs }
    }
}
#[derive(Debug)]
struct SendReaction<T> {
    send: SendMolecule<T>,
    refs: Refs<T>,
}
impl<T> SendReaction<T> {
    fn new(send: SendMolecule<T>, refs: Refs<T>) -> Self {
        SendReaction { send, refs }
    }
}
#[derive(Debug)]
struct ChoiceReaction<T> {
    molecules: Vec<Molecule<T>>,
    refs: Refs<T>,
}
impl<T> ChoiceReaction<T> {
    fn new(molecules: Vec<Molecule<T>>, refs: Refs<T>) -> Self {
        ChoiceReaction { molecules, refs }
    }
    fn collapse(self, name: &Name<T>, is_read: bool) -> (Molecule<T>, Refs<T>) {
        let ChoiceReaction { mut molecules, refs } = self;
        let molecule = molecules.drain(..)
            .filter(|x| {
                match *x {
                    Molecule::Read(ref read) => is_read && refs.get(read.name()) == *name,
                    Molecule::Send(ref send) => !is_read && refs.get(send.name()) == *name,
                }
            })
            .nth(0)
            .unwrap();
        (molecule, refs)
    }
    fn collapse_read(self, name: &Name<T>) -> ReadReaction<T> {
        let (molecule, refs) = self.collapse(name, true);
        match molecule {
            Molecule::Read(read) => ReadReaction::new(read, refs),
            Molecule::Send(_) => unreachable!(),
        }
    }
    fn collapse_send(self, name: &Name<T>) -> SendReaction<T> {
        let (molecule, refs) = self.collapse(name, false);
        match molecule {
            Molecule::Read(_) => unreachable!(),
            Molecule::Send(send) => SendReaction::new(send, refs),
        }
    }
}
#[derive(Debug)]
struct ReactionMap<T> {
    reads: HashMap<Name<T>, Vec<Reaction<T>>>,
    sends: HashMap<Name<T>, Vec<Reaction<T>>>,
    pairs: HashMap<Name<T>, (Vec<Reaction<T>>, Vec<Reaction<T>>)>,
}
impl<T> ReactionMap<T> {
    fn new() -> Self {
        ReactionMap {
            reads: HashMap::new(),
            sends: HashMap::new(),
            pairs: HashMap::new(),
        }
    }
    fn add_choice(&mut self, choice: ChoiceReaction<T>) {
        let choice = Rc::new(choice);
        for m in &choice.molecules {
            let reaction = Reaction::Choice(choice.clone());
            match *m {
                Molecule::Read(ref read) => self.add_reaction(choice.refs.get(read.name()), reaction, true),
                Molecule::Send(ref send) => self.add_reaction(choice.refs.get(send.name()), reaction, false),
            }
        }
    }
    fn add_reaction(&mut self, name: Name<T>, reaction: Reaction<T>, is_read: bool) {
        if let Some(&mut (ref mut reads, ref mut sends)) = self.pairs.get_mut(&name) {
            select(is_read, reads, sends).push(reaction);
            return;
        }
        if let Some(that) = select(is_read, &mut self.sends, &mut self.reads).remove(&name) {
            let pair = if is_read { (vec![reaction], that) } else { (that, vec![reaction]) };
            self.pairs.insert(name, pair);
        }
        else {
            select(is_read, &mut self.reads, &mut self.sends)
                .entry(name).or_insert(Vec::new()).push(reaction);
        }
    }
    fn add_read(&mut self, read: ReadReaction<T>) {
        let name = read.refs.get(read.read.name());
        let reaction = Reaction::Read(read);
        self.add_reaction(name, reaction, true);
    }
    fn add_send(&mut self, send: SendReaction<T>) {
        let name = send.refs.get(send.send.name());
        let reaction = Reaction::Send(send);
        self.add_reaction(name, reaction, false);
    }
    fn collapse_read(&mut self, reaction: Reaction<T>, name: &Name<T>) -> ReadReaction<T> {
        match reaction {
            Reaction::Choice(choice) => self.unwrap_choice(choice).collapse_read(name),
            Reaction::Read(read) => read,
            _ => unreachable!(),
        }
    }
    fn collapse_send(&mut self, reaction: Reaction<T>, name: &Name<T>) -> SendReaction<T> {
        match reaction {
            Reaction::Choice(choice) => self.unwrap_choice(choice).collapse_send(name),
            Reaction::Send(send) => send,
            _ => unreachable!(),
        }
    }
    fn remove_option(set: &mut Vec<Reaction<T>>, choice: &Rc<ChoiceReaction<T>>) {
        set.retain(|x| {
            match *x {
                Reaction::Choice(ref c) => !Rc::ptr_eq(c, choice),
                _ => true,
            }
        })
    }
    fn remove_choice(&mut self, name: &Name<T>, choice: &Rc<ChoiceReaction<T>>, is_read: bool) {
        if let Some((mut reads, mut sends)) = self.pairs.remove(&name) {
            Self::remove_option(select(is_read, &mut reads, &mut sends), choice);
            if select(is_read, &mut reads, &mut sends).is_empty() {
                select(is_read, &mut self.reads, &mut self.sends)
                    .insert(name.clone(), select(is_read, sends, reads));
            }
            else {
                self.pairs.insert(name.clone(), (reads, sends));
            }
        }
        else if let Some(mut set) = select(is_read, &mut self.reads, &mut self.sends).remove(name) {
            Self::remove_option(&mut set, choice);
            if !set.is_empty() {
                select(is_read, &mut self.reads, &mut self.sends)
                    .insert(name.clone(), set);
            }
        }
    }
    fn unwrap_choice(&mut self, choice: Rc<ChoiceReaction<T>>) -> ChoiceReaction<T> {
        for m in &choice.molecules {
            match *m {
                Molecule::Read(ref read) => self.remove_choice(read.name(), &choice, true),
                Molecule::Send(ref send) => self.remove_choice(send.name(), &choice, false),
            }
        }
        Rc::try_unwrap(choice).ok().unwrap()
    }
    fn next(&mut self) -> Option<(ReadReaction<T>, SendReaction<T>)> {
        let name = match self.pairs.keys().nth(0) {
            Some(name) => Some(name.clone()),
            None => None,
        };
        if let Some(ref name) = name {
            if let Some((mut reads, mut sends)) = self.pairs.remove(name) {
                let read = reads.remove(0);
                let send = sends.remove(0);
                match (reads.is_empty(), sends.is_empty()) {
                    (false, false) => { self.pairs.insert(name.clone(), (reads, sends)); },
                    (false, true) => { self.reads.insert(name.clone(), reads); },
                    (true, false) => { self.sends.insert(name.clone(), sends); },
                    (true, true) => {},
                };
                let read = self.collapse_read(read, name);
                let send = self.collapse_send(send, name);
                return Some((read, send))
            }
        }
        None
    }
}
#[inline]
fn select<T>(condition: bool, this: T, that: T) -> T {
    if condition { this } else { that }
}
