use ::Mods;
use ::Name;
use ::OnRead;
use ::OnSend;
use ::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub enum Molecule<T> {
    Read(ReadMolecule<T>),
    Send(SendMolecule<T>),
}
impl<T> Clone for Molecule<T> {
    fn clone(&self) -> Self {
        match self {
            Molecule::Read(x) => Molecule::Read(x.clone()),
            Molecule::Send(x) => Molecule::Send(x.clone()),
        }
    }
}
#[derive(Debug)]
pub struct ReadMolecule<T> {
    name: Name,
    read: Rc<dyn OnRead<T>>,
}
impl<T> ReadMolecule<T> {
    pub fn new(name: Name, read: Rc<dyn OnRead<T>>) -> Self {
        ReadMolecule { name, read }
    }
    #[inline]
    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn read(self, mods: &mut Mods<T>, refs: Refs, names: Vec<Name>) {
        let read = self.read.clone();
        read.read(mods, self, refs, names);
    }
}
impl<T> Clone for ReadMolecule<T> {
    fn clone(&self) -> Self {
        ReadMolecule {
            name: self.name.clone(),
            read: self.read.clone(),
        }
    }
}
#[derive(Debug)]
pub struct SendMolecule<T> {
    name: Name,
    send: Rc<dyn OnSend<T>>,
}
impl<T> SendMolecule<T> {
    pub fn new(name: Name, send: Rc<dyn OnSend<T>>) -> Self {
        SendMolecule { name, send }
    }
    #[inline]
    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn send(self, mods: &mut Mods<T>, refs: Refs) -> Vec<Name> {
        let send = self.send.clone();
        send.send(mods, self, refs)
    }
}
impl<T> Clone for SendMolecule<T> {
    fn clone(&self) -> Self {
        SendMolecule {
            name: self.name.clone(),
            send: self.send.clone(),
        }
    }
}
impl<T> From<ReadMolecule<T>> for Molecule<T> {
    fn from(read: ReadMolecule<T>) -> Self {
        Molecule::Read(read)
    }
}
impl<T> From<SendMolecule<T>> for Molecule<T> {
    fn from(send: SendMolecule<T>) -> Self {
        Molecule::Send(send)
    }
}
