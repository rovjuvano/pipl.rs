use crate::name::Name;
use crate::prefix::Prefix;
use std::collections::BTreeMap;
use std::rc::Rc;
#[derive(Debug)]
pub(crate) struct ChoiceContext<T> {
    pub map: Rc<BTreeMap<Name, Name>>,
    pub prefixes: Rc<Vec<Rc<Prefix<T>>>>,
}
impl<T> ChoiceContext<T> {
    pub fn get_name(&self, key: &Name) -> Name {
        self.map.get(key).unwrap_or(key).clone()
    }
}
impl<T> Clone for ChoiceContext<T> {
    fn clone(&self) -> Self {
        ChoiceContext {
            map: self.map.clone(),
            prefixes: self.prefixes.clone(),
        }
    }
}
