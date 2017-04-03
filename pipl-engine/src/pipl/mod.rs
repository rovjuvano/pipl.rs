pub mod mods;
use self::mods::Mods;

use ::channel::Channel;
use ::name::Name;
use ::process::sequence::Sequence;
use ::reaction::Reaction;
use ::reaction::sequence::SequenceReaction;
use ::refs::Refs;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
pub struct Pipl {
    map: ReactionMap,
}
impl Pipl {
    pub fn new() -> Pipl {
        Pipl {
            map: ReactionMap::new(),
        }
    }
    pub fn add(&mut self, sequence: Sequence) {
        let channel = sequence.channel().clone();
        let reaction = Reaction::new_sequence(Refs::new(), Rc::new(sequence));
        self.add_reaction(&channel, Rc::new(reaction));
    }
    pub fn step(&mut self) {
        if let Some((reader, sender)) = self.map.next() {
            let mut mods = Mods::new();
            let output = sender.output(&mut mods);
            reader.input(&mut mods, output);
            mods.apply(self);
        }
    }
    fn add_reaction(&mut self, channel: &Channel, reaction: Rc<Reaction>) {
        self.map.add(channel, reaction);
    }
}
#[derive(Debug)]
struct ReactionMap {
    map: HashMap<Channel, ReactionQueue>,
    queue: ReadyQueue,
}
impl ReactionMap {
    fn new() -> Self {
        ReactionMap {
            map: HashMap::new(),
            queue: ReadyQueue::new(),
        }
    }
    fn add(&mut self, channel: &Channel, reaction: Rc<Reaction>) {
        self.map.entry(channel.clone())
            .or_insert(ReactionQueue::new())
            .add(reaction);
        if let Some(q) = self.map.get(&channel.invert()) {
            if q.is_ready() {
                self.queue.add(channel.name().clone());
            }
        }
    }
    fn collapse(&mut self, channel: &Channel) -> SequenceReaction {
        let reaction = self.map.get_mut(channel).unwrap().next();
        if Rc::strong_count(&reaction) > 1 {
            for c in reaction.channels() {
                self.remove(&c.translate(reaction.refs()), reaction.refs());
            }
        }
        match Rc::try_unwrap(reaction).unwrap() {
            Reaction::Choice(c)   => c.collapse(channel),
            Reaction::Sequence(s) => s,
        }
    }
    fn next(&mut self) -> Option<(SequenceReaction, SequenceReaction)> {
        if let Some(name) = self.queue.next() {
            let read = &Channel::Read(name.clone());
            let send = &Channel::Send(name.clone());
            if self.still_ready(read, send) {
                Some((self.collapse(read), self.collapse(send)))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
    fn remove(&mut self, channel: &Channel, refs: &Refs) {
        if let Some(queue) = self.map.get_mut(channel) {
            queue.remove(refs);
        }
    }
    fn still_ready(&self, read: &Channel, send: &Channel) -> bool {
        match (self.map.get(read), self.map.get(send)) {
            (Some(read_q), Some(send_q)) => read_q.is_ready() && send_q.is_ready(),
            _ => false,
        }
    }
}
#[derive(Debug)]
struct ReactionQueue(Vec<Rc<Reaction>>);
impl ReactionQueue {
    fn new() -> Self {
        ReactionQueue(Vec::new())
    }
    fn add(&mut self, reaction: Rc<Reaction>) {
        self.0.push(reaction);
    }
    fn is_ready(&self) -> bool {
        self.0.len() > 0
    }
    fn next(&mut self) -> Rc<Reaction> {
        self.0.remove(0)
    }
    fn remove(&mut self, refs: &Refs) {
        if let Some(i) = self.0.iter().position(|x| {
            ::ptr_eq(x.refs(), refs)
        }) {
            self.0.remove(i);
        }
    }
}
#[derive(Debug)]
struct ReadyQueue(Vec<Name>);
impl ReadyQueue {
    fn new() -> Self {
        ReadyQueue(Vec::new())
    }
    fn add(&mut self, name: Name) {
        self.0.push(name);
    }
    fn next(&mut self) -> Option<Name> {
        if self.0.len() > 0 {
            Some(self.0.remove(0))
        } else {
            None
        }
    }
}
