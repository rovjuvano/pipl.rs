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
    pub fn read(self, mods: &mut Mods, names: Vec<Name>) {
        let SequenceReaction { refs, prefix } = self;
        Self::react(mods, refs, prefix, Some(names));
    }
    pub fn send(self, mods: &mut Mods) -> Vec<Name> {
        let SequenceReaction { refs, prefix } = self;
        Self::react(mods, refs, prefix, None).unwrap_or_else(|| Vec::new())
    }
    fn react(mods: &mut Mods, mut refs: Refs, prefix: Rc<Prefix>, read_names: Option<Vec<Name>>) -> Option<Vec<Name>> {
        let mut send_names = None;
        for action in prefix.actions() {
            match action {
                &Action::Repeat => mods.add_sequence(refs.clone(), prefix.clone()),
                &Action::Restrict(ref names) => refs.new_names(names.clone()),
                &Action::Communicate(ref names) => {
                    match prefix.channel() {
                        &Channel::Read(_) => refs.set_names(names.clone(), read_names.clone().unwrap()),
                        &Channel::Send(_) => send_names = Some(refs.get_names(&names)),
                    }
                },
                &Action::Call(ref call) => refs = call.call(refs),
                &Action::Prefix(ref prefix) => mods.add_sequence(refs.clone(), prefix.clone()),
                &Action::Parallel(ref sequences) => mods.add_parallel(refs.clone(), sequences.clone()),
                &Action::Choice(ref sequences) => mods.add_choice(refs.clone(), sequences.clone()),
            }
        }
        send_names
    }
    #[inline]
    pub fn refs(&self) -> &Refs {
        &self.refs
    }
}
