pub mod mods;
use self::mods::Mods;

use ::channel::Channel;
use ::name::Name;
use ::prefix::Prefix;
use ::reaction::Reaction;
use ::reaction::sequence::SequenceReaction;
use ::refs::Refs;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
pub struct Pipl<T> {
    map: ReactionMap<T>,
}
impl<T> Pipl<T> {
    pub fn new() -> Self {
        Pipl {
            map: ReactionMap::new(),
        }
    }
    pub fn add(&mut self, prefix: Prefix<T>) {
        let channel = prefix.channel().clone();
        let reaction = Reaction::new_sequence(Refs::new(), Rc::new(prefix));
        self.add_reaction(&channel, Rc::new(reaction));
    }
    pub fn step(&mut self) {
        if let Some((reader, sender)) = self.map.next() {
            let mut mods = Mods::new();
            let output = sender.send(&mut mods);
            reader.read(&mut mods, output);
            mods.apply(self);
        }
    }
    fn add_reaction(&mut self, channel: &Channel<T>, reaction: Rc<Reaction<T>>) {
        self.map.add(channel, reaction);
    }
}
#[derive(Debug)]
struct ReactionMap<T> {
    map: HashMap<Channel<T>, ReactionQueue<T>>,
    queue: ReadyQueue<T>,
}
impl<T> ReactionMap<T> {
    fn new() -> Self {
        ReactionMap {
            map: HashMap::new(),
            queue: ReadyQueue::new(),
        }
    }
    fn add(&mut self, channel: &Channel<T>, reaction: Rc<Reaction<T>>) {
        self.map.entry(channel.clone())
            .or_insert(ReactionQueue::new())
            .add(reaction);
        if let Some(q) = self.map.get(&channel.invert()) {
            if q.is_ready() {
                self.queue.add(channel.name().clone());
            }
        }
    }
    fn collapse(&mut self, channel: &Channel<T>) -> SequenceReaction<T> {
        let reaction = self.map.get_mut(channel).unwrap().next();
        if Rc::strong_count(&reaction) > 1 {
            for c in reaction.channels() {
                self.remove(&c.translate(reaction.refs()), reaction.refs());
            }
        }
        match Rc::try_unwrap(reaction).ok().unwrap() {
            Reaction::Choice(c)   => c.collapse(channel),
            Reaction::Sequence(s) => s,
        }
    }
    fn next(&mut self) -> Option<(SequenceReaction<T>, SequenceReaction<T>)> {
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
    fn remove(&mut self, channel: &Channel<T>, refs: &Refs<T>) {
        if let Some(queue) = self.map.get_mut(channel) {
            queue.remove(refs);
        }
    }
    fn still_ready(&self, read: &Channel<T>, send: &Channel<T>) -> bool {
        match (self.map.get(read), self.map.get(send)) {
            (Some(read_q), Some(send_q)) => read_q.is_ready() && send_q.is_ready(),
            _ => false,
        }
    }
}
#[derive(Debug)]
struct ReactionQueue<T>(Vec<Rc<Reaction<T>>>);
impl<T> ReactionQueue<T> {
    fn new() -> Self {
        ReactionQueue(Vec::new())
    }
    fn add(&mut self, reaction: Rc<Reaction<T>>) {
        self.0.push(reaction);
    }
    fn is_ready(&self) -> bool {
        self.0.len() > 0
    }
    fn next(&mut self) -> Rc<Reaction<T>> {
        self.0.remove(0)
    }
    fn remove(&mut self, refs: &Refs<T>) {
        if let Some(i) = self.0.iter().position(|x| {
            ::std::ptr::eq(x.refs(), refs)
        }) {
            self.0.remove(i);
        }
    }
}
#[derive(Debug)]
struct ReadyQueue<T>(Vec<Name<T>>);
impl<T> ReadyQueue<T> {
    fn new() -> Self {
        ReadyQueue(Vec::new())
    }
    fn add(&mut self, name: Name<T>) {
        self.0.push(name);
    }
    fn next(&mut self) -> Option<Name<T>> {
        if self.0.len() > 0 {
            Some(self.0.remove(0))
        } else {
            None
        }
    }
}
