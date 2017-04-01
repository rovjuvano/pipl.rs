use ::name::Name;
use ::process::Process;
#[derive(Debug)]
pub struct Names {
    names: Vec<Name>,
    suffix: Process,
}
impl Names {
    pub fn new(names: Vec<Name>, suffix: Process) -> Self {
        Names { names: names, suffix: suffix }
    }
    #[inline]
    pub fn is_nonterminal(&self) -> bool {
        self.suffix.is_nonterminal()
    }
    #[inline]
    pub fn names(&self) -> &Vec<Name> {
        &self.names
    }
    #[inline]
    pub fn suffix(&self) -> &Process {
        &self.suffix
    }
}
