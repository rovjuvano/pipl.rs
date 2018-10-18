use ::channel::Channel;
use ::pipl::Pipl;
use ::prefix::Prefix;
use ::reaction::Reaction;
use ::refs::Refs;
use std::clone::Clone;
use std::rc::Rc;
#[derive(Debug)]
pub struct Mods<T> {
    new: Vec<(Channel<T>, Rc<Reaction<T>>)>,
}
impl<T> Mods<T> {
    pub fn new() -> Self {
        Mods { new: Vec::new() }
    }
    pub fn apply(self, pipl: &mut Pipl<T>) {
        for (channel, reaction) in self.new.into_iter() {
            pipl.add_reaction(&channel, reaction);
        }
    }
    pub fn add_choice(&mut self, refs: Refs<T>, sequences: Vec<Rc<Prefix<T>>>) {
        let channels: Vec<_> = sequences.iter().map(|s|
            s.channel().translate(&refs)
        ).collect();
        let reaction = Rc::new(Reaction::new_choice(refs, sequences.clone()));
        for c in channels.into_iter() {
            self.new.push((c, reaction.clone()));
        }
    }
    pub fn add_parallel(&mut self, refs: Refs<T>, sequences: Vec<Rc<Prefix<T>>>) {
        if let Some((last, head)) = sequences.split_last() {
            for s in head.iter() {
                self.add_sequence(refs.clone(), s.clone());
            }
            self.add_sequence(refs, last.clone());
        }
    }
    pub fn add_sequence(&mut self, refs: Refs<T>, sequence: Rc<Prefix<T>>) {
        let channel = sequence.channel().translate(&refs);
        let reaction = Reaction::new_sequence(refs, sequence);
        self.new.push((channel, Rc::new(reaction)));
    }
}
