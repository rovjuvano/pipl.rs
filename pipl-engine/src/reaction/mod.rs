pub mod choice;
use self::choice::ChoiceReaction;

pub mod sequence;
use self::sequence::SequenceReaction;

use ::channel::Channel;
use ::prefix::Prefix;
use ::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub enum Reaction<T> {
    Choice(ChoiceReaction<T>),
    Sequence(SequenceReaction<T>),
}
impl<T> Reaction<T> {
    pub fn new_choice(refs: Refs<T>, sequences: Vec<Rc<Prefix<T>>>) -> Self {
        Reaction::Choice(ChoiceReaction::new(refs, sequences))
    }
    pub fn new_sequence(refs: Refs<T>, sequence: Rc<Prefix<T>>) -> Self {
        Reaction::Sequence(SequenceReaction::new(refs, sequence))
    }
    pub fn channels(&self) -> Vec<&Channel<T>> {
        use self::Reaction::*;
        match self {
            &Choice(ref c)   => c.channels(),
            &Sequence(ref s) => s.channels(),
        }
    }
    pub fn refs(&self) -> &Refs<T> {
        use self::Reaction::*;
        match self {
            &Choice(ref c)   => c.refs(),
            &Sequence(ref s) => s.refs(),
        }
    }
}
