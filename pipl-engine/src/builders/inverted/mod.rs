mod choice;
use self::choice::Choice;

mod parallel;
use self::parallel::Parallel;

mod sequence;
use self::sequence::{Prefix, Read, Send, Suffix};

mod terminal;
use self::terminal::Terminal;

#[derive(Debug, Default)]
pub struct Builder {
    processes: Vec<Prefix>,
}
impl Builder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn sequence() -> Terminal {
        Terminal::Sequence
    }
    pub fn choice() -> Choice {
        Choice::new()
    }
    pub fn parallel() -> Parallel {
        Parallel::new()
    }
    fn add_read(&mut self, read: Read) {
        self.processes.push(Prefix::Read(read));
    }
    fn add_send(&mut self, send: Send) {
        self.processes.push(Prefix::Send(send));
    }
}
