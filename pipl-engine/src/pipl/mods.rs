use ::channel::Channel;
use ::pipl::Pipl;
use ::process::call_process::CallProcess;
use ::process::choice::ChoiceProcess;
use ::process::names::Names;
use ::process::parallel::ParallelProcess;
use ::process::Process;
use ::process::sequence::Sequence;
use ::reaction::Reaction;
use ::refs::Refs;
use std::rc::Rc;
#[derive(Debug)]
pub struct Mods {
    new: Vec<(Channel, Rc<Reaction>)>,
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
            &Choice(ref p)   => self.add_choice(refs, p.clone()),
            &Names(ref p)    => self.add_names(refs, p.clone()),
            &Parallel(ref p) => self.add_parallel(refs, p.clone()),
            &Sequence(ref p) => self.add_sequence(refs, p.clone()),
            &Terminal        => {},
        }
    }
    fn add_choice(&mut self, refs: Refs, choice: Rc<ChoiceProcess>) {
        let channels: Vec<_> = choice.options().iter().map(|s|
            s.channel().translate(&refs)
        ).collect();
        let reaction = Rc::new(Reaction::new_choice(refs, choice.clone()));
        for c in channels.into_iter() {
            self.new.push((c, reaction.clone()));
        }
    }
    fn add_names(&mut self, mut refs: Refs, names: Rc<Names>) {
        refs.new_names(names.names().clone());
        self.produce(refs, names.suffix().clone())
    }
    fn add_parallel(&mut self, refs: Refs, parallel: Rc<ParallelProcess>) {
        if let Some((last, head)) = parallel.sequences().split_last() {
            for s in head.iter() {
                self.add_sequence(refs.clone(), s.clone());
            }
            self.add_sequence(refs, last.clone());
        }
    }
    pub fn add_sequence(&mut self, refs: Refs, sequence: Rc<Sequence>) {
        let channel = sequence.channel().translate(&refs);
        let reaction = Reaction::new_sequence(refs, sequence);
        self.new.push((channel, Rc::new(reaction)));
    }
    fn call(&mut self, refs: Refs, call: Rc<CallProcess>) {
        let new_refs = call.call.call(refs);
        self.produce(new_refs, &call.suffix);
    }
}
