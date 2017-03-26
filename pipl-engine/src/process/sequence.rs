use ::channel::Channel;
use ::name::Name;
use ::prefix::Prefix;
use ::process::Process;
#[derive(Debug)]
pub struct Sequence {
    prefix: Prefix,
    suffix: Process,
}
impl Sequence {
    pub fn new(prefix: Prefix, suffix: Process) -> Self {
        Sequence { prefix: prefix, suffix: suffix }
    }
    #[inline]
    pub fn channel(&self) -> &Channel {
        self.prefix.channel()
    }
    #[inline]
    pub fn is_repeating(&self) -> bool {
        self.prefix.is_repeating()
    }
    #[inline]
    pub fn names(&self) -> &Vec<Name> {
        self.prefix.names()
    }
    #[inline]
    pub fn suffix(&self) -> &Process {
        &self.suffix
    }
}
