use ::Mods;
use ::Name;
use ::OnRead;
use ::OnSend;
use ::Refs;
use std::rc::Rc;
pub struct ReadReaction {
    read: Rc<OnRead>,
    refs: Refs,
}
impl ReadReaction {
    pub fn new(read: Rc<OnRead>, refs: Refs) -> Self {
        ReadReaction { read, refs }
    }
    pub fn read(self, mods: &mut Mods, names: Vec<Name>) {
        let ReadReaction { read, refs } = self;
        read.read(mods, refs, names);
    }
}
pub struct SendReaction {
    send: Rc<OnSend>,
    refs: Refs,
}
impl SendReaction {
    pub fn new(send: Rc<OnSend>, refs: Refs) -> Self {
        SendReaction { send, refs }
    }
    pub fn send(self, mods: &mut Mods) -> Vec<Name> {
        let SendReaction { send, refs } = self;
        send.send(mods, refs)
    }
}
