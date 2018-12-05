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
pub(crate) struct Processor<'a, T: 'a> {
    contexts: &'a mut ContextStore<T>,
    names: &'a mut NameStore<T>,
}
impl<'a, T> Processor<'a, T> {
    pub fn new(contexts: &'a mut ContextStore<T>, names: &'a mut NameStore<T>) -> Self {
        Processor { contexts, names }
    }
    pub fn activate(&mut self, name: Name, reader: Context<T>, sender: Context<T>) {
        let r = self.contexts.collapse(reader, &name, PrefixDirection::Read);
        let s = self.contexts.collapse(sender, &name, PrefixDirection::Send);
        let output = self.react(s, None);
        self.react(r, output);
    }
    fn add_choice(&mut self, prefix: Rc<Prefix<T>>, ctx: ChoiceContext<T>) {
        self.contexts.add(prefix, Context::Choice(ctx));
    }
    fn add_prefix(&mut self, ctx: PrefixContext<T>) {
        self.contexts.add(ctx.prefix.clone(), Context::Prefix(ctx));
    }
    fn react(
        &mut self,
        mut ctx: PrefixContext<T>,
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
            ctx.new_names(self.names, list);
            action = iter.next();
        }
        if let Some(&Action::Communicate(ref list)) = action {
            match ctx.prefix.direction() {
                PrefixDirection::Read => ctx.set_names(list, read_names.unwrap_or_else(Vec::new)),
                PrefixDirection::Send => send_names = Some(ctx.get_names(list)),
            }
            action = iter.next();
        }
        if let Some(&Action::Call(ref call)) = action {
            call.call(CallFrame::new(&mut ctx, self.names));
            action = iter.next();
        }
        if let Some(&Action::Prefix(ref prefix)) = action {
            ctx.prefix = prefix.clone();
            self.add_prefix(ctx);
        } else {
            if let Some(&Action::Restrict(ref list)) = action {
                ctx.new_names(self.names, list);
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
