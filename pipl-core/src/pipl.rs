use atom::Atom;
use atom::AtomCreator;

#[derive(Debug)]
pub struct Pipl {
    atom_creator: AtomCreator,
}

pub fn connect() -> Pipl {
    Pipl { atom_creator: AtomCreator::new() }
}

impl Pipl {
    pub fn atom(&mut self) -> Atom {
        self.atom_creator.create()
    }
}
