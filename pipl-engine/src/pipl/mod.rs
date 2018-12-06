pub(crate) mod context;
pub(crate) mod processor;

use crate::bindings::Bindings;
use crate::name::Name;
use crate::name::NameStore;
use crate::pipl::context::Context;
use crate::pipl::context::PrefixContext;
use crate::pipl::processor::Processor;
use crate::prefix::Prefix;
use crate::prefix::PrefixDirection;
use std::any::Any;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
use std::rc::Rc;
#[derive(Debug)]
pub struct Pipl {
    contexts: ContextStore,
    names: NameStore,
}
impl Pipl {
    pub fn new() -> Self {
        Pipl {
            contexts: ContextStore::new(),
            names: NameStore::new(),
        }
    }
    pub fn add(&mut self, prefix: Prefix) {
        let p = Rc::new(prefix);
        self.contexts.add(p.clone(), Context::prefix(p));
    }
    pub fn dup_name(&mut self, name: &Name) -> Name {
        self.names.dup_name(name)
    }
    pub fn get_value<T: Any + fmt::Debug>(&self, name: &Name) -> Option<&T> {
        self.names.get_value(name)
    }
    pub fn new_name<T: Any + fmt::Debug>(&mut self, data: T) -> Name {
        self.names.new_name(data)
    }
    pub fn step(&mut self) {
        if let Some((name, reader, sender)) = self.contexts.next() {
            Processor::new(&mut self.contexts, &mut self.names).activate(name, reader, sender);
        }
    }
}
#[derive(Debug)]
pub(crate) struct ContextStore {
    set: ContextSet,
    ready: ReadySet,
}
impl ContextStore {
    fn new() -> Self {
        ContextStore {
            set: ContextSet::new(),
            ready: ReadySet::new(),
        }
    }
    fn add(&mut self, prefix: Rc<Prefix>, context: Context) {
        let name = context.get_name(prefix.name());
        self.set.add(name.clone(), prefix.direction(), context);
        if self.set.is_waiting(&name) {
            self.ready.add(name);
        }
    }
    fn collapse(
        &mut self,
        context: Context,
        name: &Name,
        direction: PrefixDirection,
    ) -> PrefixContext {
        match context {
            Context::Choice(ctx) => {
                let mut r = None;
                for p in ctx.prefixes.iter() {
                    let n = ctx.bindings.get_name(p.name());
                    if r.is_none() && *name == n && direction == p.direction() {
                        r = Some(p);
                    } else {
                        self.set.remove(&n, p.direction(), &*ctx.bindings);
                    }
                }
                PrefixContext::new(r.unwrap().clone(), Rc::try_unwrap(ctx.bindings).unwrap())
            }
            Context::Prefix(ctx) => ctx,
        }
    }
    fn next(&mut self) -> Option<(Name, Context, Context)> {
        if let Some(name) = self.ready.next() {
            let (read, send) = self.set.next(&name);
            Some((name, read, send))
        } else {
            None
        }
    }
}
#[derive(Debug, Default)]
struct ContextSet {
    set: BTreeMap<Name, (Vec<Context>, Vec<Context>)>,
}
impl ContextSet {
    fn new() -> Self {
        ContextSet {
            set: BTreeMap::new(),
        }
    }
    fn add(&mut self, name: Name, direction: PrefixDirection, context: Context) {
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
        if let Some((reads, sends)) = self.set.get(name) {
            !reads.is_empty() && !sends.is_empty()
        } else {
            false
        }
    }
    fn next(&mut self, name: &Name) -> (Context, Context) {
        let (mut reads, mut sends) = self.set.remove(name).unwrap();
        (reads.remove(0), sends.remove(0))
    }
    fn remove(&mut self, name: &Name, direction: PrefixDirection, bindings: &Bindings) {
        if let Some((reads, sends)) = self.set.get_mut(name) {
            let queue = match direction {
                PrefixDirection::Read => reads,
                PrefixDirection::Send => sends,
            };
            if let Some(i) = queue.iter().position(|x| match x {
                Context::Choice(ctx) => ::std::ptr::eq(&*ctx.bindings, bindings),
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
