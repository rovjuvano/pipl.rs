use ::channel::Channel;
use ::name::Name;
use ::process::mods::Mods;
use ::process::sequence::Sequence;
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
        let reaction = SequenceReaction::new(Refs::new(), Rc::new(sequence));
        self.map.add(&channel, reaction);
    }
    pub fn step(&mut self) {
        if let Some((reader, sender)) = self.map.next() {
            let mut pipl = Mods::new();
            let output = sender.output(&mut pipl);
            reader.input(&mut pipl, output);
        }
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
    fn add(&mut self, channel: &Channel, reaction: SequenceReaction) {
        self.map.entry(channel.clone())
            .or_insert(ReactionQueue::new())
            .add(reaction);
        if let Some(q) = self.map.get(&channel.invert()) {
            if q.is_ready() {
                self.queue.add(channel.name().clone());
            }
        }
    }
    fn next(&mut self) -> Option<(SequenceReaction, SequenceReaction)> {
        if let Some(name) = self.queue.next() {
            let reader = self.map.get_mut(&Channel::Read(name.clone())).unwrap().next();
            let sender = self.map.get_mut(&Channel::Send(name.clone())).unwrap().next();
            Some((reader, sender))
        } else {
            None
        }
    }
}
#[derive(Debug)]
struct ReactionQueue(Vec<SequenceReaction>);
impl ReactionQueue {
    fn new() -> Self {
        ReactionQueue(Vec::new())
    }
    fn add(&mut self, reaction: SequenceReaction) {
        self.0.push(reaction);
    }
    fn is_ready(&self) -> bool {
        self.0.len() > 0
    }
    fn next(&mut self) -> SequenceReaction {
        self.0.remove(0)
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
