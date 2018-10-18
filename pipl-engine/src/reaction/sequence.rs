use ::channel::Channel;
use ::name::Name;
use ::pipl::mods::Mods;
use ::prefix::Action;
use ::prefix::Prefix;
use ::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub struct SequenceReaction<T> {
    refs: Refs<T>,
    prefix: Rc<Prefix<T>>,
}
impl<T> SequenceReaction<T> {
    pub fn new(refs: Refs<T>, prefix: Rc<Prefix<T>>) -> Self {
        SequenceReaction { refs: refs, prefix: prefix }
    }
    pub fn channels(&self) -> Vec<&Channel<T>> {
        vec![self.prefix.channel()]
    }
    pub fn read(self, mods: &mut Mods<T>, names: Vec<Name<T>>) {
        let SequenceReaction { refs, prefix } = self;
        Self::react(mods, refs, prefix, Some(names));
    }
    pub fn send(self, mods: &mut Mods<T>) -> Vec<Name<T>> {
        let SequenceReaction { refs, prefix } = self;
        Self::react(mods, refs, prefix, None).unwrap_or_else(|| Vec::new())
    }
    fn react(mods: &mut Mods<T>, mut refs: Refs<T>, prefix: Rc<Prefix<T>>, read_names: Option<Vec<Name<T>>>) -> Option<Vec<Name<T>>> {
        let mut send_names = None;
        let mut iter = prefix.actions().iter();
        let mut action = iter.next();
        if let Some(&Action::Repeat) = action {
            mods.add_sequence(refs.clone(), prefix.clone());
            action = iter.next();
        }
        if let Some(&Action::Restrict(ref names)) = action {
            refs.new_names(names.clone());
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
            refs = call.call(refs);
            action = iter.next();
        }
        if let Some(&Action::Prefix(ref prefix)) = action {
            mods.add_sequence(refs, prefix.clone());
        }
        else {
            if let Some(&Action::Restrict(ref names)) = action {
                refs.new_names(names.clone());
                action = iter.next();
            }
            if let Some(&Action::Parallel(ref sequences)) = action {
                mods.add_parallel(refs, sequences.clone());
            }
            else if let Some(&Action::Choice(ref sequences)) = action {
                mods.add_choice(refs, sequences.clone());
            }
        }
        send_names
    }
    #[inline]
    pub fn refs(&self) -> &Refs<T> {
        &self.refs
    }
}
