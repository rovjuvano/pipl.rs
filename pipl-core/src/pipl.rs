use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;
use atom::Atom;
use atom::AtomCreator;
use self::PiplAtom::{Pos,Neg};

// #[derive(Debug)]
pub struct Pipl {
    atom_creator: AtomCreator,
    atom_queues: Vec<PiplAtom>,
}

struct PosPiplAtom {
    atom: Atom,
    func: Box<Fn(&mut Pipl, &mut Vec<Atom>)>
}
impl Debug for PosPiplAtom {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Pos:{:?}", self.atom)
    }
}
struct NegPiplAtom {
    atom: Atom,
    func: Box<Fn(&mut Pipl, &Vec<Atom>)>
}
impl Debug for NegPiplAtom {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Neg:{:?}", self.atom)
    }
}
enum PiplAtom {
    Pos(PosPiplAtom),
    Neg(NegPiplAtom),
}
impl Debug for PiplAtom {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (case, atom) = match self {
            ref atom @ &Pos(_) => ("Pos", atom),
            ref atom @ &Neg(_) => ("Neg", atom),
        };
        write!(f, "{}:{:?}", case, atom)
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
        where T: Fn(&mut Pipl, &mut Vec<Atom>) + 'static {
        self.add_atom(PiplAtom::Pos(PosPiplAtom {
            atom: atom,
            func: Box::new(func)
        }));
    }
    pub fn add_negative<T>(&mut self, atom: Atom, func: T)
        where T: Fn(&mut Pipl, &Vec<Atom>) + 'static {
        self.add_atom(PiplAtom::Neg(NegPiplAtom {
            atom: atom,
            func: Box::new(func)
        }));
    }
    fn add_atom(&mut self, atom: PiplAtom) {
        match (atom, if self.atom_queues.len() > 0 { self.atom_queues.pop() } else { None::<PiplAtom> }) {
            (Pos(pos), Some(Neg(neg))) | (Neg(neg), Some(Pos(pos))) => self.react(pos, neg),
            (atom, _) => self.atom_queues.push(atom),
        }
    }
    fn react(&mut self, pos: PosPiplAtom, neg: NegPiplAtom) {
        println!("{:?} -> {:?}", pos, neg);
        let mut args = vec![];
        (pos.func)(self, &mut args);
        (neg.func)(self, &args);
    }
}
