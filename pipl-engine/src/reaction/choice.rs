use ::channel::Channel;
use ::prefix::Prefix;
use ::reaction::sequence::SequenceReaction;
use ::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub struct ChoiceReaction<T> {
    refs: Refs<T>,
    sequences: Vec<Rc<Prefix<T>>>,
}
impl<T> ChoiceReaction <T>{
    pub fn new(refs: Refs<T>, sequences: Vec<Rc<Prefix<T>>>) -> Self {
        ChoiceReaction { refs: refs, sequences: sequences }
    }
    pub fn channels(&self) -> Vec<&Channel<T>> {
        self.sequences.iter().map(|x| x.channel()).collect()
    }
    pub fn collapse(self, channel: &Channel<T>) -> SequenceReaction<T> {
        let ChoiceReaction { refs, sequences } = self;
        let s = sequences.iter()
            .filter(|x| x.channel().translate(&refs) == *channel)
            .nth(0)
            .unwrap();
        SequenceReaction::new(refs, s.clone())
    }
    #[inline]
    pub fn refs(&self) -> &Refs<T> {
        &self.refs
    }
}
