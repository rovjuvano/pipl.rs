use crate::call::Call;
use crate::name::Name;
use std::rc::Rc;
#[derive(Debug)]
pub enum Action<T> {
    Repeat,
    Restrict(Vec<Name>),
    Communicate(Vec<Name>),
    Call(Rc<dyn Call<T>>),
    Prefix(Rc<Prefix<T>>),
    Parallel(Vec<Rc<Prefix<T>>>),
    Choice(Vec<Rc<Prefix<T>>>),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrefixDirection {
    Read,
    Send,
}
#[derive(Debug)]
pub struct Prefix<T> {
    actions: Vec<Action<T>>,
    direction: PrefixDirection,
    name: Name,
}
impl<T> Prefix<T> {
    pub fn new(name: Name, direction: PrefixDirection, actions: Vec<Action<T>>) -> Self {
        Prefix {
            actions,
            direction,
            name,
        }
    }
    #[inline]
    pub fn actions(&self) -> &Vec<Action<T>> {
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
