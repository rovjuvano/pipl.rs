use ::molecule::{ReadMolecule, SendMolecule};
use ::Name;
use ::Mods;
use ::Refs;
use std::fmt;
pub trait OnRead<T>: fmt::Debug {
    fn read(&self, mods: &mut Mods<T>, read: ReadMolecule<T>, refs: Refs<T>, names: Vec<Name<T>>);
}
pub trait OnSend<T>: fmt::Debug {
    fn send(&self, mods: &mut Mods<T>, send: SendMolecule<T>, refs: Refs<T>) -> Vec<Name<T>>;
}
