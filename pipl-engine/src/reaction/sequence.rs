use crate::call::CallFrame;
use crate::channel::Channel;
use crate::name::Name;
use crate::pipl::mods::Mods;
use crate::prefix::Action;
use crate::prefix::Prefix;
use crate::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub struct SequenceReaction<T> {
    refs: Refs,
    prefix: Rc<Prefix<T>>,
}
impl<T> SequenceReaction<T> {
    pub fn new(refs: Refs, prefix: Rc<Prefix<T>>) -> Self {
        SequenceReaction {
            refs: refs,
            prefix: prefix,
        }
    }
    pub fn channels(&self) -> Vec<&Channel> {
        vec![self.prefix.channel()]
    }
    pub fn read(self, mods: &mut Mods<T>, names: Vec<Name>) {
        let SequenceReaction { refs, prefix } = self;
        Self::react(mods, refs, prefix, Some(names));
    }
    pub fn send(self, mods: &mut Mods<T>) -> Vec<Name> {
        let SequenceReaction { refs, prefix } = self;
        Self::react(mods, refs, prefix, None).unwrap_or_else(|| Vec::new())
    }
    fn react(
        mods: &mut Mods<T>,
        mut refs: Refs,
        prefix: Rc<Prefix<T>>,
        read_names: Option<Vec<Name>>,
    ) -> Option<Vec<Name>> {
        let mut send_names = None;
        let mut iter = prefix.actions().iter();
        let mut action = iter.next();
        if let Some(&Action::Repeat) = action {
            mods.add_sequence(refs.clone(), prefix.clone());
            action = iter.next();
        }
        if let Some(&Action::Restrict(ref names)) = action {
            mods.new_names(&mut refs, names);
            action = iter.next();
        }
        if let Some(&Action::Communicate(ref names)) = action {
            match prefix.channel() {
                &Channel::Read(_) => refs.set_names(names.clone(), read_names.unwrap()),
                &Channel::Send(_) => send_names = Some(refs.get_names(&names)),
            }
            action = iter.next();
        }
        if let Some(&Action::Call(ref call)) = action {
            call.call(CallFrame::new(&mut refs, mods));
            action = iter.next();
        }
        if let Some(&Action::Prefix(ref prefix)) = action {
            mods.add_sequence(refs, prefix.clone());
        } else {
            if let Some(&Action::Restrict(ref names)) = action {
                mods.new_names(&mut refs, names);
                action = iter.next();
            }
            if let Some(&Action::Parallel(ref sequences)) = action {
                mods.add_parallel(refs, sequences.clone());
            } else if let Some(&Action::Choice(ref sequences)) = action {
                mods.add_choice(refs, sequences.clone());
            }
        }
        send_names
    }
    #[inline]
    pub fn refs(&self) -> &Refs {
        &self.refs
    }
}
