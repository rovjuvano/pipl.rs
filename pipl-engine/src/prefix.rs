use crate::call::Call;
use crate::channel::Channel;
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
#[derive(Debug)]
pub struct Prefix<T> {
    channel: Channel,
    actions: Vec<Action<T>>,
}
impl<T> Prefix<T> {
    pub fn new(channel: Channel, actions: Vec<Action<T>>) -> Self {
        Prefix {
            channel: channel,
            actions: actions,
        }
    }
    #[inline]
    pub fn channel(&self) -> &Channel {
        &self.channel
    }
    #[inline]
    pub fn actions(&self) -> &Vec<Action<T>> {
        &self.actions
    }
}
