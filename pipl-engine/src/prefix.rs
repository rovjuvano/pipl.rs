use ::call::Call;
use ::channel::Channel;
use ::name::Name;
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
#[derive(Debug)]
pub struct Prefix {
    channel: Channel,
    actions: Vec<Action>,
}
impl Prefix {
    pub fn new(channel: Channel, actions: Vec<Action>) -> Prefix {
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
    pub fn actions(&self) -> &Vec<Action> {
        &self.actions
    }
}
