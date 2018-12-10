use crate::bindings::Bindings;
use crate::name::Name;
use crate::pipl::context::ContextId;
use crate::pipl::context::ContextIndex;
use crate::prefix::Prefix;
use crate::prefix::PrefixDirection;
#[derive(Debug)]
struct ChoiceContext {
    pub bindings: Bindings,
    pub prefixes: Vec<Prefix>,
}
#[derive(Debug)]
pub(in pipl) struct PrefixContext {
    pub bindings: Bindings,
    pub prefix: Prefix,
}
#[derive(Debug)]
enum Context {
    Choice(ChoiceContext),
    Prefix(PrefixContext),
}
#[derive(Debug)]
pub(in pipl) struct ContextStore {
    data: Vec<Option<Context>>,
    free: Vec<usize>,
    index: ContextIndex,
}
impl ContextStore {
    pub fn new() -> Self {
        ContextStore {
            data: Vec::new(),
            free: Vec::new(),
            index: ContextIndex::new(),
        }
    }
    pub fn add_choice(&mut self, bindings: Bindings, prefixes: Vec<Prefix>) {
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
    pub fn add_prefix(&mut self, bindings: Bindings, prefix: Prefix) {
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
    pub fn next(&mut self) -> Option<(PrefixContext, PrefixContext)> {
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
