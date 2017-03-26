pub mod choice;
use self::choice::ChoiceReaction;

pub mod sequence;
use self::sequence::SequenceReaction;

use ::channel::Channel;
use ::process::choice::ChoiceProcess;
use ::process::sequence::Sequence;
use ::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub enum Reaction {
    Choice(ChoiceReaction),
    Sequence(SequenceReaction),
}
impl Reaction {
    pub fn new_choice(refs: Refs, choice: Rc<ChoiceProcess>) -> Self {
        Reaction::Choice(ChoiceReaction::new(refs, choice))
    }
    pub fn new_sequence(refs: Refs, sequence: Rc<Sequence>) -> Self {
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
