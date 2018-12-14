use crate::bindings::Bindings;
use crate::name::Name;
use crate::name::NameStore;
use std::collections::BTreeMap;
use std::fmt;
pub trait Call: fmt::Debug {
    fn call(&self, frame: CallFrame);
}
#[derive(Debug)]
pub struct CallFrame<'a> {
    bindings: &'a mut Bindings,
    names: &'a mut NameStore,
}
impl<'a> CallFrame<'a> {
    pub(crate) fn new(bindings: &'a mut Bindings, names: &'a mut NameStore) -> Self {
        CallFrame { bindings, names }
    }
    pub fn get_name(&self, name: &Name) -> Name {
        self.bindings.get_name(name)
    }
    pub fn new_name(&mut self) -> Name {
        self.names.new_name()
    }
    pub fn set_name(&mut self, key: Name, value: Name) {
        self.bindings.set_name(key, value);
    }
    /// for testing
    pub fn clone_refs(&self) -> BTreeMap<Name, Name> {
        self.bindings.clone_refs()
    }
}
