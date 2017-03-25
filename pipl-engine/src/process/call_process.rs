use ::call::Call;
use ::process::Process;
use std::rc::Rc;
#[derive(Debug)]
pub struct CallProcess {
    pub call: Rc<Call>,
    pub suffix: Process,
}
impl CallProcess {
    pub fn new(call: Rc<Call>, suffix: Process) -> Self {
        CallProcess { call: call, suffix: suffix }
    }
}
