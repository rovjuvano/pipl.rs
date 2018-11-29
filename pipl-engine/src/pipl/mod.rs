pub(crate) mod context;
pub(crate) mod processor;

use crate::channel::Channel;
use crate::name::Name;
use crate::name::NameStore;
use crate::pipl::context::Context;
use crate::pipl::processor::Processor;
use crate::prefix::Prefix;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
pub struct Pipl<T> {
    contexts: ContextStore<T>,
    names: NameStore<T>,
}
impl<T> Pipl<T> {
    pub fn new() -> Self {
        Pipl {
            contexts: ContextStore::new(),
            names: NameStore::new(),
        }
    }
    pub fn add(&mut self, prefix: Prefix<T>) {
        let channel = prefix.channel().clone();
        let context = Context::prefix(Rc::new(prefix));
        self.contexts.add(&channel, context);
    }
    pub fn dup_name(&mut self, name: &Name) -> Name {
        self.names.dup_name(name)
    }
    pub fn get_value(&self, name: &Name) -> Option<&T> {
        self.names.get_value(name)
    }
    pub fn new_name(&mut self, data: T) -> Name {
        self.names.new_name(data)
    }
    pub fn step(&mut self) {
        if let Some((name, reader, sender)) = self.contexts.next() {
            Processor::new(&mut self.contexts, &mut self.names).activate(name, reader, sender);
        }
    }
}
#[derive(Debug)]
pub(crate) struct ContextStore<T> {
    map: HashMap<Channel, ContextQueue<T>>,
    queue: ReadyQueue,
}
impl<T> ContextStore<T> {
    fn new() -> Self {
        ContextStore {
            map: HashMap::new(),
            queue: ReadyQueue::new(),
        }
    }
    fn add(&mut self, channel: &Channel, context: Context<T>) {
        self.map
            .entry(channel.clone())
            .or_insert(ContextQueue::new())
            .add(context);
        if let Some(q) = self.map.get(&channel.invert()) {
            if q.is_ready() {
                self.queue.add(channel.name().clone());
            }
        }
    }
    fn dequeue(&mut self, channel: &Channel) -> Context<T> {
        self.map.get_mut(channel).unwrap().next()
    }
    fn next(&mut self) -> Option<(Name, Context<T>, Context<T>)> {
        if let Some(name) = self.queue.next() {
            let read = &Channel::Read(name.clone());
            let send = &Channel::Send(name.clone());
            if self.still_ready(read, send) {
                Some((name, self.dequeue(read), self.dequeue(send)))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
    fn remove(&mut self, channel: &Channel, refs: &BTreeMap<Name, Name>) {
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
struct ContextQueue<T>(Vec<Context<T>>);
impl<T> ContextQueue<T> {
    fn new() -> Self {
        ContextQueue(Vec::new())
    }
    fn add(&mut self, context: Context<T>) {
        self.0.push(context);
    }
    fn is_ready(&self) -> bool {
        self.0.len() > 0
    }
    fn next(&mut self) -> Context<T> {
        self.0.remove(0)
    }
    fn remove(&mut self, refs: &BTreeMap<Name, Name>) {
        if let Some(i) = self.0.iter().position(|x| {
            if let Context::Choice(ctx) = x {
                ::std::ptr::eq(&*ctx.map, refs)
            } else {
                false
            }
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
