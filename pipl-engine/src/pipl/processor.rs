use crate::bindings::Bindings;
use crate::call::CallFrame;
use crate::name::Name;
use crate::name::NameStore;
use crate::pipl::context::ContextStore;
use crate::prefix::Action;
use crate::prefix::Prefix;
use crate::prefix::PrefixDirection;
#[derive(Debug)]
pub(in pipl) struct Processor<'a> {
    contexts: &'a mut ContextStore,
    names: &'a mut NameStore,
}
impl<'a> Processor<'a> {
    pub fn new(contexts: &'a mut ContextStore, names: &'a mut NameStore) -> Self {
        Processor { contexts, names }
    }
    pub fn react(
        &mut self,
        mut bindings: Bindings,
        prefix: Prefix,
        read_names: Option<Vec<Name>>,
    ) -> Option<Vec<Name>> {
        let mut send_names = None;
        let mut iter = prefix.actions().iter();
        let mut action = iter.next();
        if let Some(&Action::Repeat) = action {
            self.contexts.add_prefix(bindings.clone(), prefix.clone());
            action = iter.next();
        }
        if let Some(&Action::Restrict(ref list)) = action {
            restrict(&mut bindings, self.names, list);
            action = iter.next();
        }
        if let Some(&Action::Communicate(ref list)) = action {
            match prefix.direction() {
                PrefixDirection::Read => bindings.set_names(list, read_names.unwrap_or_default()),
                PrefixDirection::Send => send_names = Some(bindings.get_names(list)),
            }
            action = iter.next();
        }
        if let Some(&Action::Call(ref call)) = action {
            call.call(CallFrame::new(&mut bindings, self.names));
            action = iter.next();
        }
        if let Some(&Action::Prefix(ref prefix)) = action {
            self.contexts.add_prefix(bindings.clone(), prefix.clone());
        } else {
            if let Some(&Action::Restrict(ref list)) = action {
                restrict(&mut bindings, self.names, list);
                action = iter.next();
            }
            if let Some(&Action::Parallel(ref list)) = action {
                if let Some((last, head)) = list.split_last() {
                    for s in head.iter() {
                        self.contexts.add_prefix(bindings.clone(), s.clone());
                    }
                    self.contexts.add_prefix(bindings, last.clone());
                }
            } else if let Some(&Action::Choice(ref list)) = action {
                self.contexts.add_choice(bindings, list.clone());
            }
        }
        send_names
    }
}
fn restrict(bindings: &mut Bindings, names: &mut NameStore, new_names: &Vec<Name>) {
    for name in new_names.iter() {
        bindings.set_name(name.clone(), names.new_name());
    }
}
