use ::channel::Channel;
use ::name::Name;
use ::pipl::mods::Mods;
use ::prefix::Action;
use ::prefix::Prefix;
use ::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub struct SequenceReaction {
    refs: Refs,
    prefix: Rc<Prefix>,
}
impl SequenceReaction {
    pub fn new(refs: Refs, prefix: Rc<Prefix>) -> Self {
        SequenceReaction { refs: refs, prefix: prefix }
    }
    pub fn channels(&self) -> Vec<&Channel> {
        vec![self.prefix.channel()]
    }
    pub fn input(self, mods: &mut Mods, input_names: Vec<Name>) {
        let SequenceReaction { mut refs, prefix } = self;
        for action in prefix.actions() {
            match action {
                &Action::Repeat => mods.add_sequence(refs.clone(), prefix.clone()),
                &Action::Restrict(ref names) => refs.new_names(names.clone()),
                &Action::Communicate(ref names) => refs.set_names(names.clone(), input_names.clone()),
                &Action::Call(ref call) => refs = call.call(refs),
                &Action::Prefix(ref prefix) => mods.add_sequence(refs.clone(), prefix.clone()),
                &Action::Parallel(ref sequences) => mods.add_parallel(refs.clone(), sequences.clone()),
                &Action::Choice(ref sequences) => mods.add_choice(refs.clone(), sequences.clone()),
            }
        }
    }
    pub fn output(self, mods: &mut Mods) -> Vec<Name> {
        let SequenceReaction { mut refs, prefix } = self;
        let mut output_names = Vec::new();
        for action in prefix.actions() {
            match action {
                &Action::Repeat => mods.add_sequence(refs.clone(), prefix.clone()),
                &Action::Restrict(ref names) => refs.new_names(names.clone()),
                &Action::Communicate(ref names) => output_names = refs.get_names(&names.clone()),
                &Action::Call(ref call) => refs = call.call(refs),
                &Action::Prefix(ref prefix) => mods.add_sequence(refs.clone(), prefix.clone()),
                &Action::Parallel(ref sequences) => mods.add_parallel(refs.clone(), sequences.clone()),
                &Action::Choice(ref sequences) => mods.add_choice(refs.clone(), sequences.clone()),
            }
        }
        output_names
    }
    #[inline]
    pub fn refs(&self) -> &Refs {
        &self.refs
    }
}
