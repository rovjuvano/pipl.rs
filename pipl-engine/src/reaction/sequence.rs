use ::name::Name;
use ::process::mods::Mods;
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
    pub fn input(self, pipl: &mut Mods, names: Vec<Name>) {
        let SequenceReaction { mut refs, sequence } = self;
        if sequence.suffix().is_nonterminal() {
            refs.set_names(sequence.names().clone(), names);
        }
        Self::react(pipl, refs, sequence);
    }
    pub fn output(self, pipl: &mut Mods) -> Vec<Name> {
        let SequenceReaction { refs, sequence } = self;
        let names = refs.get_names(sequence.names());
        Self::react(pipl, refs, sequence);
        names
    }
    fn react(pipl: &mut Mods, refs: Refs, sequence: Rc<Sequence>) {
        pipl.produce(refs, sequence.suffix());
    }
}
