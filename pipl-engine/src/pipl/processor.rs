use crate::call::CallFrame;
use crate::channel::Channel;
use crate::name::Name;
use crate::name::NameStore;
use crate::pipl::context::ChoiceContext;
use crate::pipl::context::Context;
use crate::pipl::context::PrefixContext;
use crate::pipl::ContextStore;
use crate::prefix::Action;
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
        let r = reader.collapse(self.contexts, &Channel::read(name.clone()));
        let s = sender.collapse(self.contexts, &Channel::send(name));
        let output = self.react(s, None);
        self.react(r, output);
    }
    fn add(&mut self, channel: &Channel, ctx: Context<T>) {
        let c = channel.clone_with(ctx.get_name(channel.name()));
        self.contexts.add(&c, ctx);
    }
    fn add_choice(&mut self, channel: &Channel, ctx: ChoiceContext<T>) {
        self.add(channel, Context::Choice(ctx));
    }
    fn add_prefix(&mut self, ctx: PrefixContext<T>) {
        self.add(&ctx.prefix.channel().clone(), Context::Prefix(ctx));
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
            match ctx.prefix.channel() {
                &Channel::Read(_) => ctx.set_names(list, read_names.unwrap_or_else(Vec::new)),
                &Channel::Send(_) => send_names = Some(ctx.get_names(list)),
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
                        self.add_choice(s.channel(), choice_ctx.clone());
                    }
                    self.add_choice(last.channel(), choice_ctx);
                }
            }
        }
        send_names
    }
}
