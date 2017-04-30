use ::channel::Channel;
use ::prefix::Prefix;
use ::reaction::sequence::SequenceReaction;
use ::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub struct ChoiceReaction {
    refs: Refs,
    sequences: Vec<Rc<Prefix>>,
}
impl ChoiceReaction {
    pub fn new(refs: Refs, sequences: Vec<Rc<Prefix>>) -> Self {
        ChoiceReaction { refs: refs, sequences: sequences }
    }
    pub fn channels(&self) -> Vec<&Channel> {
        self.sequences.iter().map(|x| x.channel()).collect()
    }
    pub fn collapse(self, channel: &Channel) -> SequenceReaction {
        let ChoiceReaction { refs, sequences } = self;
        let s = sequences.iter()
            .filter(|x| x.channel().translate(&refs) == *channel)
            .nth(0)
            .unwrap();
        SequenceReaction::new(refs, s.clone())
    }
    #[inline]
    pub fn refs(&self) -> &Refs {
        &self.refs
    }
}
