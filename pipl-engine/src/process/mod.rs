pub mod call_process;
use self::call_process::CallProcess;

pub mod choice;
use self::choice::ChoiceProcess;

pub mod parallel;
use self::parallel::ParallelProcess;

pub mod sequence;
use self::sequence::Sequence;

use ::call::Call;
use ::prefix::Prefix;
use std::rc::Rc;
#[derive(Debug)]
pub enum Process {
    Call(Rc<CallProcess>),
    Choice(Rc<ChoiceProcess>),
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
    pub fn new_parallel(sequences: Vec<Rc<Sequence>>) -> Process {
        Process::Parallel(Rc::new(ParallelProcess::new(sequences)))
    }
    pub fn new_sequence(prefix: Prefix, suffix: Process) -> Process {
        Process::Sequence(Rc::new(Sequence::new(prefix, suffix)))
    }
    pub fn is_nonterminal(&self) -> bool {
        match self {
            &Process::Terminal => false,
            _ => true,
        }
    }
}
