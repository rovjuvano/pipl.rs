mod processor;

use crate::bindings::Bindings;
use crate::name::Name;
use crate::name::NameStore;
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
        self.contexts.add_prefix(Bindings::new(), Rc::new(prefix));
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
        if let Some((reader, sender)) = self.contexts.next() {
            let mut p = Processor::new(&mut self.contexts, &mut self.names);
            let output = p.react(sender.bindings, sender.prefix, None);
            p.react(reader.bindings, reader.prefix, output);
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ContextId(usize);
#[derive(Debug)]
enum Context {
    Choice(ChoiceContext),
    Prefix(PrefixContext),
}
#[derive(Debug)]
struct ChoiceContext {
    bindings: Bindings,
    prefixes: Vec<Rc<Prefix>>,
}
#[derive(Debug)]
struct PrefixContext {
    bindings: Bindings,
    prefix: Rc<Prefix>,
}
#[derive(Debug)]
pub(crate) struct ContextStore {
    data: Vec<Option<Context>>,
    free: Vec<usize>,
    index: ContextIndex,
}
impl ContextStore {
    fn new() -> Self {
        ContextStore {
            data: Vec::new(),
            free: Vec::new(),
            index: ContextIndex::new(),
        }
    }
    fn add_choice(&mut self, bindings: Bindings, prefixes: Vec<Rc<Prefix>>) {
        self.add_helper(|index, id| {
            for p in &prefixes {
                let name = bindings.get_name(p.name());
                index.add(name, p.direction(), id);
            }
            Context::Choice(ChoiceContext { bindings, prefixes })
        });
    }
    fn add_helper<F>(&mut self, index_and_return: F)
    where
        F: FnOnce(&mut ContextIndex, ContextId) -> Context,
    {
        match self.free.pop() {
            Some(id) => {
                let ctx = index_and_return(&mut self.index, ContextId(id));
                self.data[id] = Some(ctx);
            }
            None => {
                let id = self.data.len();
                let ctx = index_and_return(&mut self.index, ContextId(id));
                self.data.push(Some(ctx));
            }
        }
    }
    fn add_prefix(&mut self, bindings: Bindings, prefix: Rc<Prefix>) {
        self.add_helper(|index, id| {
            let name = bindings.get_name(prefix.name());
            index.add(name, prefix.direction(), id);
            Context::Prefix(PrefixContext { bindings, prefix })
        });
    }
    fn collapse(
        &mut self,
        context_id: ContextId,
        name: &Name,
        direction: PrefixDirection,
    ) -> PrefixContext {
        match self.remove(context_id).unwrap() {
            Context::Choice(ctx) => {
                let mut r = None;
                for p in ctx.prefixes.into_iter() {
                    let n = ctx.bindings.get_name(p.name());
                    if r.is_none() && *name == n && direction == p.direction() {
                        r = Some(p);
                    } else {
                        self.index.remove(&n, p.direction(), &context_id);
                    }
                }
                PrefixContext {
                    bindings: ctx.bindings,
                    prefix: r.unwrap(),
                }
            }
            Context::Prefix(ctx) => ctx,
        }
    }
    fn next(&mut self) -> Option<(PrefixContext, PrefixContext)> {
        if let Some((name, read, send)) = self.index.next() {
            let reader = self.collapse(read, &name, PrefixDirection::Read);
            let sender = self.collapse(send, &name, PrefixDirection::Send);
            Some((reader, sender))
        } else {
            None
        }
    }
    fn remove(&mut self, id: ContextId) -> Option<Context> {
        if id.0 < self.data.len() {
            match self.data[id.0].take() {
                ctx @ Some(_) => {
                    self.free.push(id.0);
                    ctx
                }
                None => None,
            }
        } else {
            None
        }
    }
}
#[derive(Debug, Default)]
struct ContextIndex {
    queues: BTreeMap<Name, (Vec<ContextId>, Vec<ContextId>)>,
    ready: BTreeSet<Name>,
}
impl ContextIndex {
    fn new() -> Self {
        ContextIndex {
            queues: BTreeMap::new(),
            ready: BTreeSet::new(),
        }
    }
    fn add(&mut self, name: Name, direction: PrefixDirection, context_id: ContextId) {
        let (reads, sends) = self
            .queues
            .entry(name.clone())
            .or_insert_with(|| (Vec::new(), Vec::new()));
        match direction {
            PrefixDirection::Read => reads.push(context_id),
            PrefixDirection::Send => sends.push(context_id),
        };
        if !reads.is_empty() && !sends.is_empty() {
            self.ready.insert(name);
        }
    }
    fn next(&mut self) -> Option<(Name, ContextId, ContextId)> {
        let maybe = self.ready.iter().next().cloned();
        if let Some(name) = maybe {
            self.ready.remove(&name);
            let (mut reads, mut sends) = self.queues.remove(&name).unwrap();
            Some((name, reads.remove(0), sends.remove(0)))
        } else {
            None
        }
    }
    fn remove(&mut self, name: &Name, direction: PrefixDirection, context_id: &ContextId) {
        if let Some((reads, sends)) = self.queues.get_mut(name) {
            let queue = match direction {
                PrefixDirection::Read => reads,
                PrefixDirection::Send => sends,
            };
            if let Some(i) = queue.iter().position(|x| x == context_id) {
                queue.remove(i);
            }
        }
    }
}
