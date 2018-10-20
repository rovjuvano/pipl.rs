use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;
use atom::Atom;
use atom::AtomCreator;
use self::PiplFunc::{Pos,Neg};

// #[derive(Debug)]
pub struct Pipl {
    atom_creator: AtomCreator,
    atom_queues: Vec<PiplAtom>,
}

enum PiplFunc {
    Pos(Box<dyn Fn(&mut Pipl) -> Vec<Atom>>),
    Neg(Box<dyn Fn(&mut Pipl, Vec<Atom>)>),
}
struct PiplAtom {
    atom: Atom,
    func: PiplFunc,
}
impl Debug for PiplAtom {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let case = match self.func {
            Pos(_) => "Pos",
            Neg(_) => "Neg",
        };
        write!(f, "{:?}:{:?}", case, self.atom)
    }
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
        where T: Fn(&mut Pipl) -> Vec<Atom> + 'static {
        self.add_atom(PiplAtom {
            atom: atom,
            func: PiplFunc::Pos(Box::new(func)),
        });
    }
    pub fn add_negative<T>(&mut self, atom: Atom, func: T)
        where T: Fn(&mut Pipl, Vec<Atom>) + 'static {
        self.add_atom(PiplAtom {
            atom: atom,
            func: PiplFunc::Neg(Box::new(func)),
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
        println!("{:?} <-> {:?}", a0, a1);
        match (a0.func , a1.func) {
            (Pos(pos_func), Neg(neg_func)) |
            (Neg(neg_func), Pos(pos_func)) => {
                let args = (pos_func)(self);
                (neg_func)(self, args);
            },
            (_, _) => panic!("woops"),
        }
    }
}
