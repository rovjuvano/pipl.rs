use ::call::Call;
use ::channel::Channel;
use ::name::Name;
use std::rc::Rc;
#[derive(Debug)]
pub enum Action<T> {
    Repeat,
    Restrict(Vec<Name<T>>),
    Communicate(Vec<Name<T>>),
    Call(Rc<dyn Call<T>>),
    Prefix(Rc<Prefix<T>>),
    Parallel(Vec<Rc<Prefix<T>>>),
    Choice(Vec<Rc<Prefix<T>>>),
}
#[derive(Debug)]
pub struct Prefix<T> {
    channel: Channel<T>,
    actions: Vec<Action<T>>,
}
impl<T> Prefix<T> {
    pub fn new(channel: Channel<T>, actions: Vec<Action<T>>) -> Self {
        Prefix {
            channel: channel,
            actions: actions,
        }
    }
    #[inline]
    pub fn channel(&self) -> &Channel<T> {
        &self.channel
    }
    #[inline]
    pub fn actions(&self) -> &Vec<Action<T>> {
        &self.actions
    }
}
