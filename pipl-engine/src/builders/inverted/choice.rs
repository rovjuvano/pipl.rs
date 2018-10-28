use super::Prefix;
use super::Read;
use super::Send;
use super::Terminal;
#[derive(Debug, Default)]
pub struct Choice {
    processes: Vec<Prefix>,
}
impl Choice {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn prefix(self) -> Terminal {
        Terminal::Choice(self)
    }
    pub(super) fn add_read(&mut self, read: Read) {
        self.processes.push(Prefix::Read(read));
    }
    pub(super) fn add_send(&mut self, send: Send) {
        self.processes.push(Prefix::Send(send));
    }
}
