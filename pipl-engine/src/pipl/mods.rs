use ::Name;
use ::OnRead;
use ::OnSend;
use ::Pipl;
use ::Refs;
use std::rc::Rc;
pub struct Mods {
    read: Rc<OnRead>,
    send: Rc<OnSend>,
    mods: Vec<Mod>,
}
enum Mod {
    Read(Read),
    Send(Send),
}
struct Read {
    name: Name,
    refs: Refs,
    read: Rc<OnRead>,
}
struct Send {
    name: Name,
    refs: Refs,
    send: Rc<OnSend>,
}
impl Mods {
    pub fn new(read: Rc<OnRead>, send: Rc<OnSend>) -> Mods {
        Mods {
            read: read,
            send: send,
            mods: Vec::new(),
        }
    }
    pub fn read(&mut self, name: &Name, refs: Refs, read: Rc<OnRead>) {
        self.mods.push(Mod::Read(Read{ name: name.clone(), read, refs }));
    }
    pub fn send(&mut self, name: &Name, refs: Refs, send: Rc<OnSend>) {
        self.mods.push(Mod::Send(Send { name: name.clone(), send, refs }));
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
        for x in self.mods.into_iter() {
            match x {
                Mod::Read(Read { name, read, refs }) => pipl.add_read(refs.get(&name).clone(), read, refs),
                Mod::Send(Send { name, send, refs }) => pipl.add_send(refs.get(&name).clone(), send, refs),
            }
        }
    }
}
