use ::molecule::{ReadMolecule, SendMolecule};
use ::Name;
use ::Mods;
use ::Refs;
use std::fmt;
pub trait OnRead: fmt::Debug {
    fn read(&self, mods: &mut Mods, read: ReadMolecule, refs: Refs, names: Vec<Name>);
}
pub trait OnSend: fmt::Debug {
    fn send(&self, mods: &mut Mods, send: SendMolecule, refs: Refs) -> Vec<Name>;
}
