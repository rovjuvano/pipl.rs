use crate::bindings::Bindings;
use crate::call::CallFrame;
use crate::name::Name;
use crate::name::NameStore;
use crate::pipl::context::ChoiceContext;
use crate::pipl::context::Context;
use crate::pipl::context::PrefixContext;
use crate::pipl::ContextStore;
use crate::prefix::Action;
use crate::prefix::Prefix;
use crate::prefix::PrefixDirection;
use std::rc::Rc;
#[derive(Debug)]
pub(crate) struct Processor<'a> {
    contexts: &'a mut ContextStore,
    names: &'a mut NameStore,
}
impl<'a> Processor<'a> {
    pub fn new(contexts: &'a mut ContextStore, names: &'a mut NameStore) -> Self {
        Processor { contexts, names }
    }
    pub fn activate(&mut self, name: Name, reader: Context, sender: Context) {
        let r = self.contexts.collapse(reader, &name, PrefixDirection::Read);
        let s = self.contexts.collapse(sender, &name, PrefixDirection::Send);
        let output = self.react(s, None);
        self.react(r, output);
    }
    fn add_choice(&mut self, prefix: Rc<Prefix>, ctx: ChoiceContext) {
        self.contexts.add(prefix, Context::Choice(ctx));
    }
    fn add_prefix(&mut self, ctx: PrefixContext) {
        self.contexts.add(ctx.prefix.clone(), Context::Prefix(ctx));
    }
    fn react(
        &mut self,
        mut ctx: PrefixContext,
        read_names: Option<Vec<Name>>,
    ) -> Option<Vec<Name>> {
        let mut send_names = None;
        let prefix = ctx.prefix.clone();
        let mut iter = prefix.actions().iter();
        let mut action = iter.next();
        if let Some(&Action::Repeat) = action {
            self.add_prefix(ctx.clone());
            action = iter.next();
        }
        if let Some(&Action::Restrict(ref list)) = action {
            restrict(&mut ctx.bindings, self.names, list);
            action = iter.next();
        }
        if let Some(&Action::Communicate(ref list)) = action {
            match ctx.prefix.direction() {
                PrefixDirection::Read => {
                    ctx.bindings.set_names(list, read_names.unwrap_or_default())
                }
                PrefixDirection::Send => send_names = Some(ctx.bindings.get_names(list)),
            }
            action = iter.next();
        }
        if let Some(&Action::Call(ref call)) = action {
            call.call(CallFrame::new(&mut ctx.bindings, self.names));
            action = iter.next();
        }
        if let Some(&Action::Prefix(ref prefix)) = action {
            ctx.prefix = prefix.clone();
            self.add_prefix(ctx);
        } else {
            if let Some(&Action::Restrict(ref list)) = action {
                restrict(&mut ctx.bindings, self.names, list);
                action = iter.next();
            }
            if let Some(&Action::Parallel(ref list)) = action {
                if let Some((last, head)) = list.split_last() {
                    for s in head.iter() {
                        self.add_prefix(ctx.clone_with(s.clone()));
                    }
                    ctx.prefix = last.clone();
                    self.add_prefix(ctx);
                }
            } else if let Some(&Action::Choice(ref list)) = action {
                let choice_ctx = ctx.choice(list.clone());
                if let Some((last, head)) = list.split_last() {
                    for s in head.iter() {
                        self.add_choice(s.clone(), choice_ctx.clone());
                    }
                    self.add_choice(last.clone(), choice_ctx);
                }
            }
        }
        send_names
    }
}
fn restrict(bindings: &mut Bindings, names: &mut NameStore, new_names: &Vec<Name>) {
    for name in new_names.iter() {
        bindings.set_name(name.clone(), names.dup_name(name));
    }
}
