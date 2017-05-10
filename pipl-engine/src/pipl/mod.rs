pub mod mods;

use ::Name;
use ::Mods;
use ::Molecule;
use ::ReadMolecule;
use ::Refs;
use ::SendMolecule;
use std::collections::HashMap;
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
    fn remove_molecule(&mut self, molecule: Molecule, refs: Refs) {
        match molecule {
            Molecule::Read(read) => self.map.remove_read(read.name(), refs),
            Molecule::Send(send) => self.map.remove_send(send.name(), refs),
        }
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
struct ReactionMap {
    reads: HashMap<Name, Vec<ReadReaction>>,
    sends: HashMap<Name, Vec<SendReaction>>,
    pairs: HashMap<Name, (Vec<ReadReaction>, Vec<SendReaction>)>,
}
impl ReactionMap {
    fn new() -> Self {
        ReactionMap {
            reads: HashMap::new(),
            sends: HashMap::new(),
            pairs: HashMap::new(),
        }
    }
    fn add_read(&mut self, read: ReadReaction) {
        let name = read.refs.get(&read.read.name());
        if let Some(&mut (ref mut reads, _)) = self.pairs.get_mut(&name) {
            reads.push(read);
            return;
        }
        if let Some(sends) = self.sends.remove(&name) {
            self.pairs.insert(name, (vec![read], sends));
        }
        else {
            self.reads.entry(name).or_insert(Vec::new()).push(read);
        }
    }
    fn add_send(&mut self, send: SendReaction) {
        let name = send.refs.get(&send.send.name());
        if let Some(&mut (_, ref mut sends)) = self.pairs.get_mut(&name) {
            sends.push(send);
            return;
        }
        if let Some(reads) = self.reads.remove(&name) {
            self.pairs.insert(name, (reads, vec![send]));
        }
        else {
            self.sends.entry(name).or_insert(Vec::new()).push(send);
        }
    }
    fn remove_read(&mut self, name: &Name, refs: Refs) {
        if let Some((mut reads, sends)) = self.pairs.remove(&name) {
            reads.retain(|x| x.refs != refs);
            if reads.is_empty() {
                self.sends.insert(name.clone(), sends);
            }
            else {
                self.pairs.insert(name.clone(), (reads, sends));
            }
        }
        else if let Some(mut reads) = self.reads.remove(name) {
            reads.retain(|x| x.refs != refs);
            if !reads.is_empty() {
                self.reads.insert(name.clone(), reads);
            }
        }
    }
    fn remove_send(&mut self, name: &Name, refs: Refs) {
        if let Some((reads, mut sends)) = self.pairs.remove(&name) {
            sends.retain(|x| x.refs != refs);
            if sends.is_empty() {
                self.reads.insert(name.clone(), reads);
            }
            else {
                self.pairs.insert(name.clone(), (reads, sends));
            }
        }
        else if let Some(mut sends) = self.sends.remove(name) {
            sends.retain(|x| x.refs != refs);
            if !sends.is_empty() {
                self.sends.insert(name.clone(), sends);
            }
        }
    }
    fn next(&mut self) -> Option<(ReadReaction, SendReaction)> {
        let name = match self.pairs.keys().nth(0) {
            Some(name) => Some(name.clone()),
            None => None,
        };
        if let Some(name) = name {
            if let Some((mut reads, mut sends)) = self.pairs.remove(&name) {
                let read = reads.remove(0);
                let send = sends.remove(0);
                match (reads.is_empty(), sends.is_empty()) {
                    (false, false) => { self.pairs.insert(name.clone(), (reads, sends)); },
                    (false, true) => { self.reads.insert(name.clone(), reads); },
                    (true, false) => { self.sends.insert(name.clone(), sends); },
                    (true, true) => {},
                };
                return Some((read, send))
            }
        }
        None
    }
}
