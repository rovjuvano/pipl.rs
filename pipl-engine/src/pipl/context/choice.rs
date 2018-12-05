use crate::bindings::Bindings;
use crate::prefix::Prefix;
use std::rc::Rc;
#[derive(Debug)]
pub(crate) struct ChoiceContext<T> {
    pub bindings: Rc<Bindings>,
    pub prefixes: Rc<Vec<Rc<Prefix<T>>>>,
}
impl<T> Clone for ChoiceContext<T> {
    fn clone(&self) -> Self {
        ChoiceContext {
            bindings: self.bindings.clone(),
            prefixes: self.prefixes.clone(),
        }
    }
}
