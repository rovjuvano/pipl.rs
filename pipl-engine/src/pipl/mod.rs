pub(crate) mod context;
pub(crate) mod processor;

use crate::name::Name;
use crate::name::NameStore;
use crate::pipl::context::Context;
use crate::pipl::context::PrefixContext;
use crate::pipl::processor::Processor;
use crate::prefix::Prefix;
use crate::prefix::PrefixDirection;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
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
        let p = Rc::new(prefix);
        self.contexts.add(p.clone(), Context::prefix(p));
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
    set: ContextSet<T>,
    ready: ReadySet,
}
impl<T> ContextStore<T> {
    fn new() -> Self {
        ContextStore {
            set: ContextSet::new(),
            ready: ReadySet::new(),
        }
    }
    fn add(&mut self, prefix: Rc<Prefix<T>>, context: Context<T>) {
        let name = context.get_name(prefix.name());
        self.set.add(name.clone(), prefix.direction(), context);
        if self.set.is_waiting(&name) {
            self.ready.add(name);
        }
    }
    fn collapse(
        &mut self,
        context: Context<T>,
        name: &Name,
        direction: PrefixDirection,
    ) -> PrefixContext<T> {
        match context {
            Context::Choice(ctx) => {
                let mut r = None;
                for p in ctx.prefixes.iter() {
                    let n = ctx.get_name(p.name());
                    if r.is_none() && *name == n && direction == p.direction() {
                        r = Some(p);
                    } else {
                        self.set.remove(&n, p.direction(), &*ctx.map);
                    }
                }
                PrefixContext {
                    map: Rc::try_unwrap(ctx.map).unwrap(),
                    prefix: r.unwrap().clone(),
                }
            }
            Context::Prefix(ctx) => ctx,
        }
    }
    fn next(&mut self) -> Option<(Name, Context<T>, Context<T>)> {
        if let Some(name) = self.ready.next() {
            let (read, send) = self.set.next(&name);
            Some((name, read, send))
        } else {
            None
        }
    }
}
#[derive(Debug, Default)]
struct ContextSet<T> {
    set: BTreeMap<Name, (Vec<Context<T>>, Vec<Context<T>>)>,
}
impl<T> ContextSet<T> {
    fn new() -> Self {
        ContextSet {
            set: BTreeMap::new(),
        }
    }
    fn add(&mut self, name: Name, direction: PrefixDirection, context: Context<T>) {
        let (reads, sends) = self
            .set
            .entry(name)
            .or_insert_with(|| (Vec::new(), Vec::new()));
        let queue = match direction {
            PrefixDirection::Read => reads,
            PrefixDirection::Send => sends,
        };
        queue.push(context);
    }
    fn is_waiting(&self, name: &Name) -> bool {
        if let Some(x) = self.set.get(name) {
            !x.0.is_empty() && !x.1.is_empty()
        } else {
            false
        }
    }
    fn next(&mut self, name: &Name) -> (Context<T>, Context<T>) {
        let mut x = self.set.remove(name).unwrap();
        (x.0.remove(0), x.1.remove(0))
    }
    fn remove(&mut self, name: &Name, direction: PrefixDirection, refs: &BTreeMap<Name, Name>) {
        if let Some((reads, sends)) = self.set.get_mut(name) {
            let queue = match direction {
                PrefixDirection::Read => reads,
                PrefixDirection::Send => sends,
            };
            if let Some(i) = queue.iter().position(|x| match x {
                Context::Choice(ctx) => ::std::ptr::eq(&*ctx.map, refs),
                _ => false,
            }) {
                queue.remove(i);
            }
        }
    }
}
#[derive(Debug, Default)]
struct ReadySet {
    set: BTreeSet<Name>,
}
impl ReadySet {
    fn new() -> Self {
        ReadySet {
            set: BTreeSet::new(),
        }
    }
    fn add(&mut self, name: Name) {
        self.set.insert(name);
    }
    fn next(&mut self) -> Option<Name> {
        let maybe = self.set.iter().next().cloned();
        if let Some(name) = maybe {
            self.set.remove(&name);
            Some(name)
        } else {
            None
        }
    }
}
