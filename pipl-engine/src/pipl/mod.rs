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
pub struct Pipl {
    map: ReactionMap,
}
impl Pipl {
    pub fn new() -> Self {
        Pipl {
            map: ReactionMap::new(),
        }
    }
    pub fn add(&mut self, molecule: Molecule) {
        self.add_molecule(molecule, Refs::new());
    }
    pub fn read(&mut self, read: ReadMolecule) {
        self.add_read(read, Refs::new());
    }
    pub fn send(&mut self, send: SendMolecule) {
        self.add_send(send, Refs::new());
    }
    fn add_molecule(&mut self, molecule: Molecule, refs: Refs) {
        match molecule {
            Molecule::Read(read) => self.add_read(read, refs),
            Molecule::Send(send) => self.add_send(send, refs),
        }
    }
    fn add_read(&mut self, read: ReadMolecule, refs: Refs) {
        self.map.add_read(ReadReaction::new(read, refs));
    }
    fn add_send(&mut self, send: SendMolecule, refs: Refs) {
        self.map.add_send(SendReaction::new(send, refs));
    }
    fn add_choice(&mut self, molecules: Vec<Molecule>, refs: Refs) {
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
enum Reaction {
    Choice(Rc<ChoiceReaction>),
    Read(ReadReaction),
    Send(SendReaction),
}
#[derive(Debug)]
struct ReadReaction {
    read: ReadMolecule,
    refs: Refs,
}
impl ReadReaction {
    fn new(read: ReadMolecule, refs: Refs) -> Self {
        ReadReaction { read, refs }
    }
}
#[derive(Debug)]
struct SendReaction {
    send: SendMolecule,
    refs: Refs,
}
impl SendReaction {
    fn new(send: SendMolecule, refs: Refs) -> Self {
        SendReaction { send, refs }
    }
}
#[derive(Debug)]
struct ChoiceReaction {
    molecules: Vec<Molecule>,
    refs: Refs,
}
impl ChoiceReaction {
    fn new(molecules: Vec<Molecule>, refs: Refs) -> Self {
        ChoiceReaction { molecules, refs }
    }
    fn collapse_read(self, name: &Name) -> ReadReaction {
        let ChoiceReaction { mut molecules, refs } = self;
        let m = molecules.drain(..)
            .filter(|x| {
                match *x {
                    Molecule::Read(ref read) => &refs.get(read.name()) == name,
                    _ => false,
                }
            })
            .nth(0)
            .map(|x| {
                match x {
                    Molecule::Read(read) => read,
                    _ => unreachable!(),
                }
            })
            .unwrap();
        ReadReaction::new(m, refs)
    }
    fn collapse_send(self, name: &Name) -> SendReaction {
        let ChoiceReaction { mut molecules, refs } = self;
        let m = molecules.drain(..)
            .filter(|x| {
                match *x {
                    Molecule::Send(ref send) => &refs.get(send.name()) == name,
                    _ => false,
                }
            })
            .nth(0)
            .map(|x| {
                match x {
                    Molecule::Send(send) => send,
                    _ => unreachable!(),
                }
            })
            .unwrap();
        SendReaction::new(m, refs)
    }
}
#[derive(Debug)]
struct ReactionMap {
    reads: HashMap<Name, Vec<Reaction>>,
    sends: HashMap<Name, Vec<Reaction>>,
    pairs: HashMap<Name, (Vec<Reaction>, Vec<Reaction>)>,
}
impl ReactionMap {
    fn new() -> Self {
        ReactionMap {
            reads: HashMap::new(),
            sends: HashMap::new(),
            pairs: HashMap::new(),
        }
    }
    fn add_choice(&mut self, choice: ChoiceReaction) {
        let choice = Rc::new(choice);
        for m in &choice.molecules {
            let reaction = Reaction::Choice(choice.clone());
            match *m {
                Molecule::Read(ref read) => self.add_read_reaction(choice.refs.get(read.name()), reaction),
                Molecule::Send(ref send) => self.add_send_reaction(choice.refs.get(send.name()), reaction),
            }
        }
    }
    fn add_read(&mut self, read: ReadReaction) {
        let name = read.refs.get(read.read.name());
        let reaction = Reaction::Read(read);
        self.add_read_reaction(name, reaction);
    }
    fn add_read_reaction(&mut self, name: Name, reaction: Reaction) {
        if let Some(&mut (ref mut reads, _)) = self.pairs.get_mut(&name) {
            reads.push(reaction);
            return;
        }
        if let Some(sends) = self.sends.remove(&name) {
            self.pairs.insert(name, (vec![reaction], sends));
        }
        else {
            self.reads.entry(name).or_insert(Vec::new()).push(reaction);
        }
    }
    fn add_send(&mut self, send: SendReaction) {
        let name = send.refs.get(send.send.name());
        let reaction = Reaction::Send(send);
        self.add_send_reaction(name, reaction);
    }
    fn add_send_reaction(&mut self, name: Name, reaction: Reaction) {
        if let Some(&mut (_, ref mut sends)) = self.pairs.get_mut(&name) {
            sends.push(reaction);
            return;
        }
        if let Some(reads) = self.reads.remove(&name) {
            self.pairs.insert(name, (reads, vec![reaction]));
        }
        else {
            self.sends.entry(name).or_insert(Vec::new()).push(reaction);
        }
    }
    fn collapse_read(&mut self, reaction: Reaction, name: &Name) -> ReadReaction {
        match reaction {
            Reaction::Choice(choice) => self.unwrap_choice(choice).collapse_read(name),
            Reaction::Read(read) => read,
            _ => unreachable!(),
        }
    }
    fn collapse_send(&mut self, reaction: Reaction, name: &Name) -> SendReaction {
        match reaction {
            Reaction::Choice(choice) => self.unwrap_choice(choice).collapse_send(name),
            Reaction::Send(send) => send,
            _ => unreachable!(),
        }
    }
    fn remove_option(set: &mut Vec<Reaction>, choice: &Rc<ChoiceReaction>) {
        set.retain(|x| {
            match *x {
                Reaction::Choice(ref c) => !Rc::ptr_eq(c, choice),
                _ => true,
            }
        })
    }
    fn remove_read(&mut self, name: &Name, choice: &Rc<ChoiceReaction>) {
        if let Some((mut reads, sends)) = self.pairs.remove(&name) {
            Self::remove_option(&mut reads, choice);
            if reads.is_empty() {
                self.sends.insert(name.clone(), sends);
            }
            else {
                self.pairs.insert(name.clone(), (reads, sends));
            }
        }
        else if let Some(mut reads) = self.reads.remove(name) {
            Self::remove_option(&mut reads, choice);
            if !reads.is_empty() {
                self.reads.insert(name.clone(), reads);
            }
        }
    }
    fn remove_send(&mut self, name: &Name, choice: &Rc<ChoiceReaction>) {
        if let Some((reads, mut sends)) = self.pairs.remove(&name) {
            Self::remove_option(&mut sends, choice);
            if sends.is_empty() {
                self.reads.insert(name.clone(), reads);
            }
            else {
                self.pairs.insert(name.clone(), (reads, sends));
            }
        }
        else if let Some(mut sends) = self.sends.remove(name) {
            Self::remove_option(&mut sends, choice);
            if !sends.is_empty() {
                self.sends.insert(name.clone(), sends);
            }
        }
    }
    fn unwrap_choice(&mut self, choice: Rc<ChoiceReaction>) -> ChoiceReaction {
        for m in &choice.molecules {
            match *m {
                Molecule::Read(ref read) => self.remove_read(read.name(), &choice),
                Molecule::Send(ref send) => self.remove_send(send.name(), &choice),
            }
        }
        Rc::try_unwrap(choice).unwrap()
    }
    fn next(&mut self) -> Option<(ReadReaction, SendReaction)> {
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
