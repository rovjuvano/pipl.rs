use ::Name;
use ::OnRead;
use ::OnSend;
use ::Pipl;
use ::Refs;
use std::rc::Rc;
pub struct Mods {
    read: Rc<OnRead>,
    send: Rc<OnSend>,
    reads: Vec<ReadMod>,
    sends: Vec<SendMod>,
}
struct ReadMod {
    name: Name,
    refs: Refs,
    read: Rc<OnRead>,
}
struct SendMod {
    name: Name,
    refs: Refs,
    send: Rc<OnSend>,
}
impl Mods {
    pub fn new(read: Rc<OnRead>, send: Rc<OnSend>) -> Mods {
        Mods {
            read: read,
            send: send,
            reads: Vec::new(),
            sends: Vec::new(),
        }
    }
    pub fn read(&mut self, name: &Name, refs: Refs, read: Rc<OnRead>) {
        self.reads.push(ReadMod{ name: name.clone(), read, refs });
    }
    pub fn send(&mut self, name: &Name, refs: Refs, send: Rc<OnSend>) {
        self.sends.push(SendMod { name: name.clone(), send, refs });
    }
    pub fn repeat_read(&mut self, name: &Name, refs: Refs) {
        let read = self.read.clone();
        self.read(name, refs, read);
    }
    pub fn repeat_send(&mut self, name: &Name, refs: Refs) {
        let send = self.send.clone();
        self.send(name, refs, send);
    }
    pub fn apply(self, pipl: &mut Pipl) {
        for ReadMod { name, read, refs } in self.reads.into_iter() {
            pipl.add_read(refs.get(&name).clone(), read, refs);
        }
        for SendMod { name, send, refs } in self.sends.into_iter() {
            pipl.add_send(refs.get(&name).clone(), send, refs);
        }
    }
}
