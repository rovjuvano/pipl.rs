#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Atom(usize);

#[derive(Debug)]
pub struct AtomCreator(usize);

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
    fn atoms_are_distinct() {
        let mut subject = AtomCreator::new();
        assert!(subject.create() != subject.create());
    }

    #[test]
    fn atoms_can_move() {
        let subject = AtomCreator::new().create();
        let x1 = subject;
        assert_eq!(subject, x1);
    }
}
