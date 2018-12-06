use crate::bindings::Bindings;
use crate::pipl::context::ChoiceContext;
use crate::prefix::Prefix;
use std::rc::Rc;
#[derive(Debug)]
pub(crate) struct PrefixContext {
    pub bindings: Bindings,
    pub prefix: Rc<Prefix>,
}
impl PrefixContext {
    pub fn new(prefix: Rc<Prefix>, bindings: Bindings) -> Self {
        PrefixContext { bindings, prefix }
    }
    pub fn choice(self, prefixes: Vec<Rc<Prefix>>) -> ChoiceContext {
        ChoiceContext {
            bindings: Rc::new(self.bindings),
            prefixes: Rc::new(prefixes),
        }
    }
    pub fn clone_with(&self, prefix: Rc<Prefix>) -> Self {
        PrefixContext {
            bindings: self.bindings.clone(),
            prefix,
        }
    }
}
impl Clone for PrefixContext {
    fn clone(&self) -> Self {
        PrefixContext {
            bindings: self.bindings.clone(),
            prefix: self.prefix.clone(),
        }
    }
}
