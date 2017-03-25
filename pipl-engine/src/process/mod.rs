pub mod call_process;
use self::call_process::CallProcess;

pub mod sequence;
use self::sequence::Sequence;

use std::rc::Rc;
#[derive(Debug)]
pub enum Process {
    Call(Rc<CallProcess>),
    Sequence(Rc<Sequence>),
    Terminal,
}
impl Process {
    pub fn is_nonterminal(&self) -> bool {
        match self {
            &Process::Terminal => false,
            _ => true,
        }
    }
}
