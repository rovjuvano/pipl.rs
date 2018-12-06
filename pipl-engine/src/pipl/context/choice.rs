use crate::bindings::Bindings;
use crate::prefix::Prefix;
use std::rc::Rc;
#[derive(Debug)]
pub(crate) struct ChoiceContext {
    pub bindings: Rc<Bindings>,
    pub prefixes: Rc<Vec<Rc<Prefix>>>,
}
impl Clone for ChoiceContext {
    fn clone(&self) -> Self {
        ChoiceContext {
            bindings: self.bindings.clone(),
            prefixes: self.prefixes.clone(),
        }
    }
}
