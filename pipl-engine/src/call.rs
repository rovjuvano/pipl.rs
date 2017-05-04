use ::Name;
use ::Mods;
use ::Refs;
use std::fmt;
pub trait OnRead: fmt::Debug {
    fn read(&self, mods: &mut Mods, refs: Refs, names: Vec<Name>);
}
pub trait OnSend: fmt::Debug {
    fn send(&self, mods: &mut Mods, refs: Refs) -> Vec<Name>;
}
