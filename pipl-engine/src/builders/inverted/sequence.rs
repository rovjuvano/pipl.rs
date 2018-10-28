use super::Builder;
use super::Choice;
use super::Parallel;
use super::Terminal;
use std::fmt;
use Atom;
pub enum Prefix {
    Read(Read),
    Send(Send),
}
impl fmt::Debug for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Prefix::Read(x) => x.fmt(f),
            Prefix::Send(x) => x.fmt(f),
        }
    }
}
pub enum Suffix {
    Read(Box<Read>),
    Send(Box<Send>),
    Terminal(Terminal),
}
impl fmt::Debug for Suffix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suffix::Read(x) => x.fmt(f),
            Suffix::Send(x) => x.fmt(f),
            Suffix::Terminal(x) => x.fmt(f),
        }
    }
}
pub struct Read(pub(super) Atom, pub(super) Suffix);
impl Read {
    pub fn read(self, atom: Atom) -> Read {
        Read(atom, Suffix::Read(Box::new(self)))
    }
    pub fn send(self, atom: Atom) -> Send {
        Send(atom, Suffix::Read(Box::new(self)))
    }
    pub fn choice(self, choice: &mut Choice) {
        choice.add_read(self)
    }
    pub fn launch(self, pipl: &mut Builder) {
        pipl.add_read(self);
    }
    pub fn parallel(self, parallel: &mut Parallel) {
        parallel.add_read(self);
    }
}
impl fmt::Debug for Read {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Suffix::Terminal(Terminal::Sequence) = self.1 {
            write!(f, "Read({:?})", self.0)
        } else {
            f.debug_tuple("Read").field(&self.0).field(&self.1).finish()
        }
    }
}
pub struct Send(pub(super) Atom, pub(super) Suffix);
impl Send {
    pub fn read(self, atom: Atom) -> Read {
        Read(atom, Suffix::Send(Box::new(self)))
    }
    pub fn send(self, atom: Atom) -> Send {
        Send(atom, Suffix::Send(Box::new(self)))
    }
    pub fn choice(self, choice: &mut Choice) {
        choice.add_send(self)
    }
    pub fn launch(self, pipl: &mut Builder) {
        pipl.add_send(self);
    }
    pub fn parallel(self, parallel: &mut Parallel) {
        parallel.add_send(self)
    }
}
impl fmt::Debug for Send {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Suffix::Terminal(Terminal::Sequence) = self.1 {
            write!(f, "Send({:?})", self.0)
        } else {
            f.debug_tuple("Send").field(&self.0).field(&self.1).finish()
        }
    }
}
