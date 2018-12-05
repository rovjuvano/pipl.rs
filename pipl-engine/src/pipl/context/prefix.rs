use crate::bindings::Bindings;
use crate::pipl::context::ChoiceContext;
use crate::prefix::Prefix;
use std::rc::Rc;
#[derive(Debug)]
pub(crate) struct PrefixContext<T> {
    pub bindings: Bindings,
    pub prefix: Rc<Prefix<T>>,
}
impl<T> PrefixContext<T> {
    pub fn new(prefix: Rc<Prefix<T>>, bindings: Bindings) -> Self {
        PrefixContext { bindings, prefix }
    }
    pub fn choice(self, prefixes: Vec<Rc<Prefix<T>>>) -> ChoiceContext<T> {
        ChoiceContext {
            bindings: Rc::new(self.bindings),
            prefixes: Rc::new(prefixes),
        }
    }
    pub fn clone_with(&self, prefix: Rc<Prefix<T>>) -> Self {
        PrefixContext {
            bindings: self.bindings.clone(),
            prefix,
        }
    }
}
impl<T> Clone for PrefixContext<T> {
    fn clone(&self) -> Self {
        PrefixContext {
            bindings: self.bindings.clone(),
            prefix: self.prefix.clone(),
        }
    }
}
