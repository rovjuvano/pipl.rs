use atom::Atom;
use atom::AtomCreator;
use self::PiplAtom::{Pos,Neg};

// #[derive(Debug)]
pub struct Pipl {
    atom_creator: AtomCreator,
    atom_queues: Vec<PiplAtom>,
}

enum PiplAtom {
    Pos {
        atom: Atom,
        func: Box<Fn(&mut Pipl, &mut Vec<Atom>)>
    },
    Neg {
        atom: Atom,
        func: Box<Fn(&mut Pipl, &Vec<Atom>)>
    },
}

impl Pipl {
    pub fn connect<T>(func: T) where T: Fn(&mut Pipl) {
        func(&mut Pipl {
            atom_creator: AtomCreator::new(),
            atom_queues: vec![],
        });
    }
    pub fn atom(&mut self) -> Atom {
        self.atom_creator.create()
    }
    pub fn add_positive<T>(&mut self, atom: Atom, func: T)
        where T: Fn(&mut Pipl, &mut Vec<Atom>) + 'static {
        self.add_atom(Pos {
            atom: atom,
            func: Box::new(func)
        });
    }
    pub fn add_negative<T>(&mut self, atom: Atom, func: T)
        where T: Fn(&mut Pipl, &Vec<Atom>) + 'static {
        self.add_atom(Neg {
            atom: atom,
            func: Box::new(func)
        });
    }
    fn add_atom(&mut self, atom: PiplAtom) {
        if self.atom_queues.len() > 0 {
            let other = self.atom_queues.pop().unwrap();
            self.react(atom, other);
        }
        else {
            self.atom_queues.push(atom);
        }
    }
    fn react(&mut self, a0: PiplAtom, a1: PiplAtom) {
        match (a0, a1) {
            (Pos { atom: _, func: pos_func}, Neg { atom: _, func: neg_func}) |
            (Neg { atom: _, func: neg_func}, Pos { atom: _, func: pos_func}) => {
                let mut args = vec![];
                pos_func(self, &mut args);
                neg_func(self, &args);
            },
            (_, _) => panic!("at the disco"),
        };
    }
}
