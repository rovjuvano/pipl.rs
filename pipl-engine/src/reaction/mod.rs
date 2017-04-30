pub mod choice;
use self::choice::ChoiceReaction;

pub mod sequence;
use self::sequence::SequenceReaction;

use ::channel::Channel;
use ::prefix::Prefix;
use ::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub enum Reaction {
    Choice(ChoiceReaction),
    Sequence(SequenceReaction),
}
impl Reaction {
    pub fn new_choice(refs: Refs, sequences: Vec<Rc<Prefix>>) -> Self {
        Reaction::Choice(ChoiceReaction::new(refs, sequences))
    }
    pub fn new_sequence(refs: Refs, sequence: Rc<Prefix>) -> Self {
        Reaction::Sequence(SequenceReaction::new(refs, sequence))
    }
    pub fn channels(&self) -> Vec<&Channel> {
        use self::Reaction::*;
        match self {
            &Choice(ref c)   => c.channels(),
            &Sequence(ref s) => s.channels(),
        }
    }
    pub fn refs(&self) -> &Refs {
        use self::Reaction::*;
        match self {
            &Choice(ref c)   => c.refs(),
            &Sequence(ref s) => s.refs(),
        }
    }
}
