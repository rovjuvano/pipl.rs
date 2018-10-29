#![deny(bare_trait_objects)]
#![allow(unknown_lints)]
#![warn(clippy)]
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Atom(usize);
impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Atom({})", self.0)
    }
}
#[derive(Copy, Clone)]
pub struct Molecule(usize);
impl fmt::Debug for Molecule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Molecule({})", self.0)
    }
}
#[derive(Debug)]
enum AnyMolecule {
    Simple(SimpleMolecule),
    Repeating(SimpleMolecule),
    Parallel(Vec<Molecule>),
    Choice(Vec<Molecule>),
    Call(Call, Molecule),
    Terminal,
}
#[derive(Debug)]
enum SimpleMolecule {
    Read(Atom, Vec<Atom>, Molecule),
    Send(Atom, Vec<Atom>, Molecule),
}
pub struct Call(Box<dyn Fn() + 'static>);
impl fmt::Debug for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Call(Fn())")
    }
}
impl<T: Fn() + 'static> From<T> for Call {
    fn from(f: T) -> Self {
        Call(Box::new(f))
    }
}
#[derive(Debug, Default)]
pub struct Pipl {
    atoms: Vec<String>,
    solution: Solution,
}
impl Pipl {
    pub fn new() -> Self {
        Pipl {
            atoms: Vec::new(),
            solution: Solution::new(),
        }
    }
    pub fn atom<S: Into<String>>(&mut self, data: S) -> Atom {
        let id = self.atoms.len();
        self.atoms.push(data.into());
        Atom(id)
    }
    pub fn terminal(&self) -> Molecule {
        Molecule(0)
    }
    pub fn call<I: Into<Call>>(&mut self, call: I, next: Molecule) -> Molecule {
        self.solution
            .add_molecule(AnyMolecule::Call(call.into(), next))
    }
    pub fn read<'a, I>(&mut self, atom: Atom, atoms: I, next: Molecule) -> Molecule
    where
        I: IntoIterator<Item = &'a Atom>,
    {
        self.solution
            .add_molecule(AnyMolecule::Simple(SimpleMolecule::Read(
                atom,
                atoms.into_iter().cloned().collect(),
                next,
            )))
    }
    pub fn send<'a, I>(&mut self, atom: Atom, atoms: I, next: Molecule) -> Molecule
    where
        I: IntoIterator<Item = &'a Atom>,
    {
        self.solution
            .add_molecule(AnyMolecule::Simple(SimpleMolecule::Send(
                atom,
                atoms.into_iter().cloned().collect(),
                next,
            )))
    }
    pub fn repeating_read<'a, I>(&mut self, atom: Atom, atoms: I, next: Molecule) -> Molecule
    where
        I: IntoIterator<Item = &'a Atom>,
    {
        self.solution
            .add_molecule(AnyMolecule::Repeating(SimpleMolecule::Read(
                atom,
                atoms.into_iter().cloned().collect(),
                next,
            )))
    }
    pub fn repeating_send<'a, I>(&mut self, atom: Atom, atoms: I, next: Molecule) -> Molecule
    where
        I: IntoIterator<Item = &'a Atom>,
    {
        self.solution
            .add_molecule(AnyMolecule::Repeating(SimpleMolecule::Send(
                atom,
                atoms.into_iter().cloned().collect(),
                next,
            )))
    }
    pub fn parallel<'a, I>(&mut self, molecules: I) -> Molecule
    where
        I: IntoIterator<Item = &'a Molecule>,
    {
        let list = molecules.into_iter().cloned().collect();
        self.solution.add_molecule(AnyMolecule::Parallel(list))
    }
    pub fn choice<'a, I>(&mut self, molecules: I) -> Molecule
    where
        I: IntoIterator<Item = &'a Molecule>,
    {
        let list = molecules.into_iter().cloned().collect();
        self.solution.add_molecule(AnyMolecule::Choice(list))
    }
    pub fn excite(&mut self, molecule: Molecule) {
        self.solution.excite(Processor::new(molecule));
    }
    pub fn step(&mut self) {
        self.solution.step();
    }
}
#[derive(Debug, Default)]
struct Solution {
    molecules: Vec<AnyMolecule>,
    ready: ReadySet,
    reactions: ReactionSet,
}
impl Solution {
    fn new() -> Self {
        Solution {
            molecules: vec![AnyMolecule::Terminal],
            ready: ReadySet::new(),
            reactions: ReactionSet::new(),
        }
    }
    fn add_molecule(&mut self, molecule: AnyMolecule) -> Molecule {
        let id = self.molecules.len();
        self.molecules.push(molecule);
        Molecule(id)
    }
    fn excite(&mut self, mut processor: Processor) {
        #[derive(Debug)]
        enum Excite {
            Atom(Atom, bool),
            Molecule(Molecule),
            Molecules(Vec<Molecule>),
            None,
        }
        let action = if let Some(m) = self.molecules.get(processor.molecule.0) {
            match m {
                AnyMolecule::Simple(SimpleMolecule::Read(a, ..)) => Excite::Atom(*a, true),
                AnyMolecule::Simple(SimpleMolecule::Send(a, ..)) => Excite::Atom(*a, false),
                AnyMolecule::Repeating(SimpleMolecule::Read(a, ..)) => Excite::Atom(*a, true),
                AnyMolecule::Repeating(SimpleMolecule::Send(a, ..)) => Excite::Atom(*a, false),
                AnyMolecule::Parallel(p) => Excite::Molecules(p.clone()),
                AnyMolecule::Call(c, m) => {
                    c.0();
                    Excite::Molecule(*m)
                }
                AnyMolecule::Choice(..) => unimplemented!(),
                AnyMolecule::Terminal => Excite::None,
            }
        } else {
            Excite::None
        };
        match action {
            Excite::Atom(atom, is_read) => self.excite_simple(atom, processor, is_read),
            Excite::Molecule(m) => {
                processor.molecule = m;
                self.excite(processor);
            }
            Excite::Molecules(_) => unimplemented!(),
            // Excite::Molecules(list) => for m in list {
            //     self.excite(m)
            // },
            Excite::None => {}
        }
    }
    fn excite_simple(&mut self, atom: Atom, processor: Processor, is_read: bool) {
        let atom = processor.get(&atom);
        if is_read {
            self.reactions.insert_read(atom, processor);
        } else {
            self.reactions.insert_send(atom, processor);
        }
        if self.reactions.is_waiting(atom) {
            self.ready.insert(atom);
        }
    }
    pub fn step(&mut self) {
        if let Some(atom) = self.ready.next() {
            let (read, send) = self.reactions.next(atom);
            if self.reactions.is_waiting(atom) {
                self.ready.insert(atom);
            }
            self.react(read, send);
        }
    }
    fn react(&mut self, mut read: Processor, send: Processor) {
        match (
            self.molecules.get(read.molecule.0),
            self.molecules.get(send.molecule.0),
        ) {
            (
                Some(AnyMolecule::Simple(SimpleMolecule::Read(_, keys, mr))),
                Some(AnyMolecule::Simple(SimpleMolecule::Send(_, values, ms))),
            ) => {
                read.read(keys, send.send(&values));
                Some((*mr, *ms))
            }
            _ => unreachable!(),
        };
        self.excite_next(read);
        self.excite_next(send);
    }
    #[allow(needless_pass_by_value)]
    fn excite_next(&mut self, mut processor: Processor) {
        let copy = match self.molecules.get(processor.molecule.0) {
            Some(AnyMolecule::Simple(SimpleMolecule::Read(.., m))) => Some(*m),
            Some(AnyMolecule::Simple(SimpleMolecule::Send(.., m))) => Some(*m),
            Some(AnyMolecule::Repeating(SimpleMolecule::Read(.., m))) => Some(*m),
            Some(AnyMolecule::Repeating(SimpleMolecule::Send(.., m))) => Some(*m),
            Some(_) => None,
            None => None,
        };
        if let Some(m) = copy {
            processor.molecule = m;
            self.excite(processor);
        }
    }
}
#[derive(Debug)]
struct Processor {
    molecule: Molecule,
    map: BTreeMap<Atom, Atom>,
}
impl Processor {
    fn new(molecule: Molecule) -> Self {
        Processor {
            molecule,
            map: BTreeMap::new(),
        }
    }
    #[allow(trivially_copy_pass_by_ref)]
    fn get(&self, atom: &Atom) -> Atom {
        *self.map.get(atom).unwrap_or(atom)
    }
    fn read(&mut self, keys: &[Atom], values: Vec<Atom>) {
        for (k, v) in keys.iter().zip(values) {
            self.map.insert(*k, v);
        }
    }
    fn send(&self, keys: &[Atom]) -> Vec<Atom> {
        keys.iter().map(|k| self.get(k)).collect()
    }
}
#[derive(Debug, Default)]
struct ReactionSet {
    set: BTreeMap<Atom, (Vec<Processor>, Vec<Processor>)>,
}
impl ReactionSet {
    fn new() -> Self {
        ReactionSet {
            set: BTreeMap::new(),
        }
    }
    fn insert_read(&mut self, atom: Atom, processor: Processor) {
        self.set
            .entry(atom)
            .or_insert_with(|| (Vec::new(), Vec::new()))
            .0
            .push(processor);
    }
    fn insert_send(&mut self, atom: Atom, processor: Processor) {
        self.set
            .entry(atom)
            .or_insert_with(|| (Vec::new(), Vec::new()))
            .1
            .push(processor);
    }
    fn is_waiting(&self, atom: Atom) -> bool {
        if let Some(x) = self.set.get(&atom) {
            !x.0.is_empty() && !x.1.is_empty()
        } else {
            false
        }
    }
    fn next(&mut self, atom: Atom) -> (Processor, Processor) {
        let mut x = self.set.remove(&atom).unwrap();
        (x.0.remove(0), x.1.remove(0))
    }
}
#[derive(Debug, Default)]
struct ReadySet {
    set: BTreeSet<Atom>,
}
impl ReadySet {
    fn new() -> Self {
        ReadySet {
            set: BTreeSet::new(),
        }
    }
    fn insert(&mut self, atom: Atom) {
        self.set.insert(atom);
    }
    fn next(&mut self) -> Option<Atom> {
        let maybe = self.set.iter().next().cloned();
        if let Some(atom) = maybe {
            self.set.remove(&atom);
            maybe
        } else {
            None
        }
    }
}
