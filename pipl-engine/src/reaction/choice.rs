use ::channel::Channel;
use ::process::choice::ChoiceProcess;
use ::reaction::sequence::SequenceReaction;
use ::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub struct ChoiceReaction {
    refs: Refs,
    choice: Rc<ChoiceProcess>,
}
impl ChoiceReaction {
    pub fn new(refs: Refs, choice: Rc<ChoiceProcess>) -> Self {
        ChoiceReaction { refs: refs, choice: choice }
    }
    pub fn channels(&self) -> Vec<&Channel> {
        self.choice.options().iter().map(|x| x.channel()).collect()
    }
    pub fn collapse(self, channel: &Channel) -> SequenceReaction {
        let ChoiceReaction { refs, choice } = self;
        let s = choice.options().iter()
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
