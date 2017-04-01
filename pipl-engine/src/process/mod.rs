pub mod call_process;
use self::call_process::CallProcess;

pub mod choice;
use self::choice::ChoiceProcess;

pub mod names;
use self::names::Names;

pub mod parallel;
use self::parallel::ParallelProcess;

pub mod sequence;
use self::sequence::Sequence;

use ::call::Call;
use ::name::Name;
use ::prefix::Prefix;
use std::rc::Rc;
#[derive(Debug)]
pub enum Process {
    Call(Rc<CallProcess>),
    Choice(Rc<ChoiceProcess>),
    Names(Rc<Names>),
    Parallel(Rc<ParallelProcess>),
    Sequence(Rc<Sequence>),
    Terminal,
}
impl Process {
    pub fn new_call(call: Rc<Call>, suffix: Process) -> Process {
        Process::Call(Rc::new(CallProcess::new(call, suffix)))
    }
    pub fn new_choice(options: Vec<Rc<Sequence>>) -> Process {
        Process::Choice(Rc::new(ChoiceProcess::new(options)))
    }
    pub fn new_names(names: Vec<Name>, suffix: Process) -> Process {
        Process::Names(Rc::new(Names::new(names, suffix)))
    }
    pub fn new_parallel(sequences: Vec<Rc<Sequence>>) -> Process {
        Process::Parallel(Rc::new(ParallelProcess::new(sequences)))
    }
    pub fn new_sequence(names: Vec<Name>, prefix: Prefix, suffix: Process) -> Process {
        Process::Sequence(Rc::new(Sequence::new(names, prefix, suffix)))
    }
    pub fn is_nonterminal(&self) -> bool {
        match self {
            &Process::Terminal => false,
            &Process::Names(ref p) => p.is_nonterminal(),
            _ => true,
        }
    }
}
