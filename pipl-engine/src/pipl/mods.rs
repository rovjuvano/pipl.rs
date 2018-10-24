use ::channel::Channel;
use ::name::Name;
use ::name::NameStore;
use ::pipl::ReactionMap;
use ::prefix::Prefix;
use ::reaction::Reaction;
use ::refs::Refs;
use std::clone::Clone;
use std::rc::Rc;
#[derive(Debug)]
pub struct Mods<'a, T: 'a> {
    names: &'a mut NameStore<T>,
    new: Vec<(Channel, Rc<Reaction<T>>)>,
}
impl<'a, T: 'a> Mods<'a, T> {
    pub fn new(names: &'a mut NameStore<T>) -> Self {
        Mods { names, new: Vec::new() }
    }
    pub(super) fn apply(self, reactions: &mut ReactionMap<T>) {
        for (channel, reaction) in self.new.into_iter() {
            reactions.add(&channel, reaction);
        }
    }
    pub fn add_choice(&mut self, refs: Refs, sequences: Vec<Rc<Prefix<T>>>) {
        let channels: Vec<_> = sequences.iter().map(|s|
            s.channel().translate(&refs)
        ).collect();
        let reaction = Rc::new(Reaction::new_choice(refs, sequences.clone()));
        for c in channels.into_iter() {
            self.new.push((c, reaction.clone()));
        }
    }
    pub fn add_parallel(&mut self, refs: Refs, sequences: Vec<Rc<Prefix<T>>>) {
        if let Some((last, head)) = sequences.split_last() {
            for s in head.iter() {
                self.add_sequence(refs.clone(), s.clone());
            }
            self.add_sequence(refs, last.clone());
        }
    }
    pub fn add_sequence(&mut self, refs: Refs, sequence: Rc<Prefix<T>>) {
        let channel = sequence.channel().translate(&refs);
        let reaction = Reaction::new_sequence(refs, sequence);
        self.new.push((channel, Rc::new(reaction)));
    }
    pub fn get_value(&self, name: &Name) -> Option<&T> {
        self.names.get_value(name)
    }
    pub fn new_name(&mut self, value: T) -> Name {
        self.names.new_name(value)
    }
    pub fn new_names(&mut self, refs: &mut Refs, names: &[Name]) {
        for name in names.iter() {
            refs.set(name.clone(), self.names.dup_name(name));
        }
    }
}
