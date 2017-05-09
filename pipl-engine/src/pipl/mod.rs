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
    map: ReactionMap
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
    map: HashMap<Name, ReactionQueue>,
    queue: Vec<Name>,
}
impl ReactionMap {
    fn new() -> Self {
        ReactionMap {
            map: HashMap::new(),
            queue: Vec::new(),
        }
    }
    fn add_read(&mut self, reaction: ReadReaction) {
        let name = reaction.refs.get(&reaction.read.name());
        let queue = self.map
            .entry(name.clone())
            .or_insert(ReactionQueue::new());
        queue.add_read(reaction);
        if queue.is_ready() {
            self.queue.push(name);
        }
    }
    fn add_send(&mut self, reaction: SendReaction) {
        let name = reaction.refs.get(&reaction.send.name());
        let queue = self.map
            .entry(name.clone())
            .or_insert(ReactionQueue::new());
        queue.add_send(reaction);
        if queue.is_ready() {
            self.queue.push(name);
        }
    }
    fn remove_read(&mut self, name: &Name, refs: Refs) {
        if let Some(queue) = self.map.get_mut(name) {
            queue.remove_read(refs);
        }
    }
    fn remove_send(&mut self, name: &Name, refs: Refs) {
        if let Some(queue) = self.map.get_mut(name) {
            queue.remove_send(refs);
        }
    }
    fn next(&mut self) -> Option<(ReadReaction, SendReaction)> {
        if self.queue.is_empty() {
            return None
        }
        else {
            let name = self.queue.remove(0);
            if let Some(queue) = self.map.get_mut(&name) {
                if queue.is_ready() {
                    return Some(queue.take())
                }
            }
        }
        self.next()
    }
}
#[derive(Debug)]
struct ReactionQueue {
    reads: Vec<ReadReaction>,
    sends: Vec<SendReaction>,
}
impl ReactionQueue {
    fn new() -> Self {
        ReactionQueue {
            reads: Vec::new(),
            sends: Vec::new(),
        }
    }
    fn add_read(&mut self, read: ReadReaction) {
        self.reads.push(read);
    }
    fn add_send(&mut self, send: SendReaction) {
        self.sends.push(send);
    }
    fn remove_read(&mut self, refs: Refs) {
        self.reads.retain(|x| x.refs != refs);
    }
    fn remove_send(&mut self, refs: Refs) {
        self.sends.retain(|x| x.refs != refs);
    }
    fn is_ready(&self) -> bool {
        ! self.reads.is_empty() && ! self.sends.is_empty()
    }
    fn take(&mut self) -> (ReadReaction, SendReaction) {
        let read = self.reads.remove(0);
        let send = self.sends.remove(0);
        (read, send)
    }
}
