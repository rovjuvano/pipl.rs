use ::Mods;
use ::Name;
use ::OnRead;
use ::OnSend;
use ::Refs;
use std::rc::Rc;
#[derive(Clone, Debug)]
pub enum Molecule {
    Read(ReadMolecule),
    Send(SendMolecule),
}
#[derive(Clone, Debug)]
pub struct ReadMolecule {
    name: Name,
    read: Rc<OnRead>,
}
impl ReadMolecule {
    pub fn new(name: Name, read: Rc<OnRead>) -> Self {
        ReadMolecule { name, read }
    }
    #[inline]
    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn read(self, mods: &mut Mods, refs: Refs, names: Vec<Name>) {
        let read = self.read.clone();
        read.read(mods, self, refs, names);
    }
}
#[derive(Clone, Debug)]
pub struct SendMolecule {
    name: Name,
    send: Rc<OnSend>,
}
impl SendMolecule {
    pub fn new(name: Name, send: Rc<OnSend>) -> Self {
        SendMolecule { name, send }
    }
    #[inline]
    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn send(self, mods: &mut Mods, refs: Refs) -> Vec<Name> {
        let send = self.send.clone();
        send.send(mods, self, refs)
    }
}
impl From<ReadMolecule> for Molecule {
    fn from(read: ReadMolecule) -> Self {
        Molecule::Read(read)
    }
}
impl From<SendMolecule> for Molecule {
    fn from(send: SendMolecule) -> Self {
        Molecule::Send(send)
    }
}
