use ::name::Name;
use ::pipl::mods::Mods;
use ::process::sequence::Sequence;
use ::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub struct SequenceReaction {
    refs: Refs,
    sequence: Rc<Sequence>,
}
impl SequenceReaction {
    pub fn new(refs: Refs, sequence: Rc<Sequence>) -> Self {
        SequenceReaction { refs: refs, sequence: sequence }
    }
    pub fn input(self, mods: &mut Mods, names: Vec<Name>) {
        let SequenceReaction { mut refs, sequence } = self;
        if sequence.suffix().is_nonterminal() {
            refs.set_names(sequence.names().clone(), names);
        }
        Self::react(mods, refs, sequence);
    }
    pub fn output(self, mods: &mut Mods) -> Vec<Name> {
        let SequenceReaction { refs, sequence } = self;
        let names = refs.get_names(sequence.names());
        Self::react(mods, refs, sequence);
        names
    }
    fn react(mods: &mut Mods, refs: Refs, sequence: Rc<Sequence>) {
        if sequence.is_repeating() {
            mods.add_sequence(refs.clone(), sequence.clone())
        }
        mods.produce(refs, sequence.suffix());
    }
}
