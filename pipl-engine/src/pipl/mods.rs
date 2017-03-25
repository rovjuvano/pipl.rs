use ::channel::Channel;
use ::pipl::Pipl;
use ::process::call_process::CallProcess;
use ::process::Process;
use ::process::sequence::Sequence;
use ::reaction::sequence::SequenceReaction;
use ::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub struct Mods {
    new: Vec<(Channel, SequenceReaction)>,
}
impl Mods {
    pub fn new() -> Self {
        Mods { new: Vec::new() }
    }
    pub fn apply(self, pipl: &mut Pipl) {
        for (channel, reaction) in self.new.into_iter() {
            pipl.add_reaction(&channel, reaction);
        }
    }
    pub fn produce(&mut self, refs: Refs, process: &Process) {
        use Process::*;
        match process {
            &Call(ref p)     => self.call(refs, p.clone()),
            &Sequence(ref p) => self.add_sequence(refs, p.clone()),
            &Terminal        => {},
        }
    }
    fn add_sequence(&mut self, refs: Refs, sequence: Rc<Sequence>) {
        let channel = sequence.channel().translate(&refs);
        let reaction = SequenceReaction::new(refs, sequence);
        self.new.push((channel, reaction));
    }
    fn call(&mut self, refs: Refs, call: Rc<CallProcess>) {
        let new_refs = call.call.call(refs);
        self.produce(new_refs, &call.suffix);
    }
}
