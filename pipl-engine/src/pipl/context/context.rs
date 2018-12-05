use crate::name::Name;
use crate::pipl::context::ChoiceContext;
use crate::pipl::context::PrefixContext;
use crate::prefix::Prefix;
use std::collections::BTreeMap;
use std::rc::Rc;
#[derive(Debug)]
pub(crate) enum Context<T> {
    Choice(ChoiceContext<T>),
    Prefix(PrefixContext<T>),
}
impl<T> Context<T> {
    pub fn prefix(prefix: Rc<Prefix<T>>) -> Self {
        Context::Prefix(PrefixContext {
            map: BTreeMap::new(),
            prefix: prefix,
        })
    }
    pub fn get_name(&self, key: &Name) -> Name {
        match self {
            Context::Choice(ctx) => ctx.get_name(key),
            Context::Prefix(ctx) => ctx.get_name(key),
        }
    }
}
