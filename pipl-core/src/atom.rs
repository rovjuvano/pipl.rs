#[derive(Debug)]
pub struct Atom(u64);

#[derive(Debug)]
pub struct AtomCreator(u64);

impl AtomCreator {
    pub fn new() -> AtomCreator {
        AtomCreator(0)
    }
    pub fn create(&mut self) -> Atom {
        let a = Atom(self.0);
        self.0 += 1;
        a
    }
}

#[cfg(test)]
mod test {
    use super::AtomCreator;

    #[test]
    fn t() {
        let mut subject = AtomCreator::new();
        assert_eq!(0, subject.create().0);
        assert_eq!(1, subject.create().0);
    }
}
