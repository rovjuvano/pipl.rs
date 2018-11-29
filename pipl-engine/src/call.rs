use crate::name::Name;
use crate::name::NameStore;
use crate::pipl::context::PrefixContext;
use std::collections::BTreeMap;
use std::fmt::Debug;
pub trait Call<T>: Debug {
    fn call(&self, frame: CallFrame<T>);
}
#[derive(Debug)]
pub struct CallFrame<'a, T: 'a> {
    ctx: &'a mut PrefixContext<T>,
    names: &'a mut NameStore<T>,
}
impl<'a, T: 'a> CallFrame<'a, T> {
    pub(crate) fn new(ctx: &'a mut PrefixContext<T>, names: &'a mut NameStore<T>) -> Self {
        CallFrame { ctx, names }
    }
    pub fn get_name(&self, name: &Name) -> Name {
        self.ctx.get_name(name)
    }
    pub fn get_value(&self, name: &Name) -> Option<&T> {
        self.names.get_value(&self.get_name(name))
    }
    pub fn new_name(&mut self, value: T) -> Name {
        self.ctx.new_name(self.names, value)
    }
    pub fn set_name(&mut self, key: Name, value: Name) {
        self.ctx.set_name(key, value);
    }
    /// for testing
    pub fn clone_refs(&self) -> BTreeMap<Name, Name> {
        self.ctx.map.clone()
    }
}
