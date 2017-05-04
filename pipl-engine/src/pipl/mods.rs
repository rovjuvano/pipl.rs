use ::Name;
use ::OnRead;
use ::OnSend;
use ::Pipl;
use ::pipl::ReadReaction;
use ::pipl::SendReaction;
use ::Refs;
use std::rc::Rc;
pub struct Mods {
    reads: Vec<ReadReaction>,
    sends: Vec<SendReaction>,
}
impl Mods {
    pub fn new() -> Mods {
        Mods {
            reads: Vec::new(),
            sends: Vec::new(),
        }
    }
    pub fn read(&mut self, _name: &Name, refs: Refs, fun: Rc<OnRead>) {
        self.reads.push(ReadReaction::new(fun, refs));
    }
    pub fn send(&mut self, _name: &Name, refs: Refs, fun: Rc<OnSend>) {
        self.sends.push(SendReaction::new(fun, refs));
    }
    pub fn apply(self, pipl: &mut Pipl) {
        for x in self.reads.into_iter() {
            pipl.add_read(x);
        }
        for x in self.sends.into_iter() {
            pipl.add_send(x);
        }
    }
}