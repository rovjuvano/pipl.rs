pub mod mods;

use ::Name;
use ::OnRead;
use ::OnSend;
use ::Mods;
use ::Refs;
use std::collections::HashMap;
use std::rc::Rc;
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
    pub fn read(&mut self, name: &Name, read: Rc<OnRead>) {
        self.add_read(name.clone(), read, Refs::new());
    }
    pub fn send(&mut self, name: &Name, send: Rc<OnSend>) {
        self.add_send(name.clone(), send, Refs::new());
    }
    fn add_read(&mut self, name: Name, read: Rc<OnRead>, refs: Refs) {
        self.map.add_read(name, ReadReaction { read, refs });
    }
    fn add_send(&mut self, name: Name, send: Rc<OnSend>, refs: Refs) {
        self.map.add_send(name, SendReaction { send, refs });
    }
    pub fn step(&mut self) {
        if let Some((reader, sender)) = self.map.next() {
            let ReadReaction { read, refs: read_refs } = reader;
            let SendReaction { send, refs: send_refs } = sender;
            let mut mods = Mods::new(read.clone(), send.clone());
            let names = send.send(&mut mods, send_refs);
            read.read(&mut mods, read_refs, names);
            mods.apply(self);
        }
    }
}
#[derive(Debug)]
struct ReadReaction {
    read: Rc<OnRead>,
    refs: Refs,
}
#[derive(Debug)]
struct SendReaction {
    send: Rc<OnSend>,
    refs: Refs,
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
    fn add_read(&mut self, name: Name, read: ReadReaction) {
        let queue = self.map
            .entry(name.clone())
            .or_insert(ReactionQueue::new());
        queue.add_read(read);
        if queue.is_ready() {
            self.queue.push(name.clone());
        }
    }
    fn add_send(&mut self, name: Name, send: SendReaction) {
        let queue = self.map
            .entry(name.clone())
            .or_insert(ReactionQueue::new());
        queue.add_send(send);
        if queue.is_ready() {
            self.queue.push(name.clone());
        }
    }
    fn next(&mut self) -> Option<(ReadReaction, SendReaction)> {
        if self.queue.len() > 0 {
            let name = self.queue.remove(0);
            Some(self.map.get_mut(&name).unwrap().take())
        }
        else {
            None
        }
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
    fn is_ready(&self) -> bool {
        ! self.reads.is_empty() && ! self.sends.is_empty()
    }
    fn take(&mut self) -> (ReadReaction, SendReaction) {
        let read = self.reads.remove(0);
        let send = self.sends.remove(0);
        (read, send)
    }
}
