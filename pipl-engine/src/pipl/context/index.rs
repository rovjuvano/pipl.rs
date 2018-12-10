use crate::name::Name;
use crate::pipl::context::ContextId;
use crate::prefix::PrefixDirection;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
#[derive(Debug, Default)]
pub(in pipl::context) struct ContextIndex {
    queues: BTreeMap<Name, (Vec<ContextId>, Vec<ContextId>)>,
    ready: BTreeSet<Name>,
}
impl ContextIndex {
    pub fn new() -> Self {
        ContextIndex {
            queues: BTreeMap::new(),
            ready: BTreeSet::new(),
        }
    }
    pub fn add(&mut self, name: Name, direction: PrefixDirection, context_id: ContextId) {
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
    pub fn next(&mut self) -> Option<(Name, ContextId, ContextId)> {
        let maybe = self.ready.iter().next().cloned();
        if let Some(name) = maybe {
            self.ready.remove(&name);
            let (mut reads, mut sends) = self.queues.remove(&name).unwrap();
            Some((name, reads.remove(0), sends.remove(0)))
        } else {
            None
        }
    }
    pub fn remove(&mut self, name: &Name, direction: PrefixDirection, context_id: &ContextId) {
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
