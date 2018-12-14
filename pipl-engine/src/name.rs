use std::fmt;
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(usize);
impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name({:?})", self.0)
    }
}
#[derive(Debug)]
pub struct NameStore(usize);
impl NameStore {
    pub(crate) fn new() -> Self {
        NameStore(0)
    }
    pub fn new_name(&mut self) -> Name {
        let name = Name(self.0);
        self.0 += 1;
        name
    }
}
