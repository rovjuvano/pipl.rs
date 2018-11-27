use crate::name::Name;
use crate::pipl::mods::Mods;
use crate::refs::Refs;
use std::fmt::Debug;
pub trait Call<T>: Debug {
    fn call(&self, frame: CallFrame<T>);
}
#[derive(Debug)]
pub struct CallFrame<'a, 'b: 'a, T: 'b> {
    mods: &'a mut Mods<'b, T>,
    refs: &'a mut Refs,
}
impl<'a, 'b, T: 'a> CallFrame<'a, 'b, T> {
    pub(crate) fn new(refs: &'a mut Refs, mods: &'a mut Mods<'b, T>) -> Self {
        CallFrame { refs, mods }
    }
    pub fn get_name(&self, name: &Name) -> Name {
        self.refs.get(name)
    }
    pub fn get_value(&self, name: &Name) -> Option<&T> {
        self.mods.get_value(&self.get_name(name))
    }
    pub fn new_name(&mut self, value: T) -> Name {
        self.mods.new_name(value)
    }
    pub fn set_name(&mut self, key: Name, value: Name) {
        self.refs.set(key, value);
    }
    /// for testing
    pub fn clone_refs(&self) -> Refs {
        self.refs.clone()
    }
}
