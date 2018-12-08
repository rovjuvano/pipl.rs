use crate::call::Call;
use crate::name::Name;
use std::fmt;
use std::rc::Rc;
#[derive(Debug)]
pub enum Action {
    Repeat,
    Restrict(Vec<Name>),
    Communicate(Vec<Name>),
    Call(Box<dyn Call>),
    Prefix(Prefix),
    Parallel(Vec<Prefix>),
    Choice(Vec<Prefix>),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrefixDirection {
    Read,
    Send,
}
#[derive(Clone)]
pub struct Prefix {
    inner: Rc<PrefixInner>,
}
#[derive(Debug)]
pub struct PrefixInner {
    actions: Vec<Action>,
    direction: PrefixDirection,
    name: Name,
}
impl Prefix {
    pub fn new(name: Name, direction: PrefixDirection, actions: Vec<Action>) -> Self {
        Prefix {
            inner: Rc::new(PrefixInner {
                actions,
                direction,
                name,
            }),
        }
    }
    #[inline]
    pub fn actions(&self) -> &Vec<Action> {
        &self.inner.actions
    }
    #[inline]
    pub fn direction(&self) -> PrefixDirection {
        self.inner.direction
    }
    #[inline]
    pub fn name(&self) -> &Name {
        &self.inner.name
    }
}
impl fmt::Debug for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Prefix")
            .field("name", &self.inner.name)
            .field("direction", &self.inner.direction)
            .field("actions", &self.inner.actions)
            .finish()
    }
}
