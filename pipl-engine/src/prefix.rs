use crate::call::Call;
use crate::name::Name;
use std::rc::Rc;
#[derive(Debug)]
pub enum Action {
    Repeat,
    Restrict(Vec<Name>),
    Communicate(Vec<Name>),
    Call(Rc<dyn Call>),
    Prefix(Rc<Prefix>),
    Parallel(Vec<Rc<Prefix>>),
    Choice(Vec<Rc<Prefix>>),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrefixDirection {
    Read,
    Send,
}
#[derive(Debug)]
pub struct Prefix {
    actions: Vec<Action>,
    direction: PrefixDirection,
    name: Name,
}
impl Prefix {
    pub fn new(name: Name, direction: PrefixDirection, actions: Vec<Action>) -> Self {
        Prefix {
            actions,
            direction,
            name,
        }
    }
    #[inline]
    pub fn actions(&self) -> &Vec<Action> {
        &self.actions
    }
    #[inline]
    pub fn direction(&self) -> PrefixDirection {
        self.direction
    }
    #[inline]
    pub fn name(&self) -> &Name {
        &self.name
    }
}
