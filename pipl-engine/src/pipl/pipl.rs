use crate::bindings::Bindings;
use crate::name::Name;
use crate::name::NameStore;
use crate::pipl::context::ContextStore;
use crate::pipl::processor::Processor;
use crate::prefix::Prefix;
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
        self.contexts.add_prefix(Bindings::new(), prefix);
    }
    pub fn new_name(&mut self) -> Name {
        self.names.new_name()
    }
    pub fn step(&mut self) {
        if let Some((reader, sender)) = self.contexts.next() {
            let mut p = Processor::new(&mut self.contexts, &mut self.names);
            let output = p.react(sender.bindings, sender.prefix, None);
            p.react(reader.bindings, reader.prefix, output);
        }
    }
}
