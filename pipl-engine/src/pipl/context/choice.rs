use crate::channel::Channel;
use crate::name::Name;
use crate::pipl::context::PrefixContext;
use crate::pipl::ContextStore;
use crate::prefix::Prefix;
use std::collections::BTreeMap;
use std::rc::Rc;
#[derive(Debug)]
pub(crate) struct ChoiceContext<T> {
    pub(crate) map: Rc<BTreeMap<Name, Name>>,
    pub(crate) prefixes: Rc<Vec<Rc<Prefix<T>>>>,
}
impl<T> ChoiceContext<T> {
    pub fn collapse(self, contexts: &mut ContextStore<T>, channel: &Channel) -> PrefixContext<T> {
        let mut r = None;
        for p in self.prefixes.iter() {
            let c = p.channel().clone_with(self.get_name(p.channel().name()));
            contexts.remove(&c, &*self.map);
            if c == *channel && r.is_none() {
                r = Some(p);
            }
        }
        PrefixContext {
            map: Rc::try_unwrap(self.map).unwrap(),
            prefix: r.unwrap().clone(),
        }
    }
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
