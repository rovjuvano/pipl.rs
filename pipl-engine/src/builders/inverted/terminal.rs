use super::Choice;
use super::Parallel;
use super::Read;
use super::Send;
use super::Suffix;
use std::fmt;
use Atom;
pub enum Terminal {
    Choice(Choice),
    Parallel(Parallel),
    Sequence,
}
impl Terminal {
    pub fn read(self, atom: Atom) -> Read {
        Read(atom, Suffix::Terminal(self))
    }
    pub fn send(self, atom: Atom) -> Send {
        Send(atom, Suffix::Terminal(self))
    }
}
impl fmt::Debug for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Terminal::Choice(x) => x.fmt(f),
            Terminal::Parallel(x) => x.fmt(f),
            Terminal::Sequence => f.write_str("Terminal"),
        }
    }
}
