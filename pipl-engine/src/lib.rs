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
enum Reactant {
    Read(Atom, Vec<Atom>),
    Send(Atom, Vec<Atom>),
}
#[derive(Debug)]
enum Product {
    Call(Call, Molecule),
    Choice(Vec<Molecule>),
    Parallel(Vec<Molecule>),
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
        self.solution.add_product(Product::Call(call.into(), next))
    }
    pub fn read<'a, I>(&mut self, atom: Atom, atoms: I, next: Molecule) -> Molecule
    where
        I: IntoIterator<Item = &'a Atom>,
    {
        let atoms = atoms.into_iter().cloned().collect();
        self.solution
            .add_reactant(Reactant::Read(atom, atoms), next)
    }
    pub fn send<'a, I>(&mut self, atom: Atom, atoms: I, next: Molecule) -> Molecule
    where
        I: IntoIterator<Item = &'a Atom>,
    {
        let atoms = atoms.into_iter().cloned().collect();
        self.solution
            .add_reactant(Reactant::Send(atom, atoms), next)
    }
    pub fn parallel<'a, I>(&mut self, molecules: I) -> Molecule
    where
        I: IntoIterator<Item = &'a Molecule>,
    {
        let list = molecules.into_iter().cloned().collect();
        self.solution.add_product(Product::Parallel(list))
    }
    pub fn choice<'a, I>(&mut self, molecules: I) -> Molecule
    where
        I: IntoIterator<Item = &'a Molecule>,
    {
        let list = molecules.into_iter().cloned().collect();
        self.solution.add_product(Product::Choice(list))
    }
    pub fn excite(&mut self, molecule: Molecule) {
        self.solution.excite(Processor::new(molecule));
    }
    pub fn step(&mut self) {
        self.solution.step();
    }
}
#[derive(Debug)]
enum InnerMolecule {
    Reaction(Reactant, Molecule),
    Product(Product),
}
#[derive(Debug, Default)]
struct Solution {
    inner: InnerSolution,
    molecules: MoleculeStore,
}
impl Solution {
    fn new() -> Self {
        Solution {
            inner: InnerSolution::new(),
            molecules: MoleculeStore::new(),
        }
    }
    fn add_reactant(&mut self, reactant: Reactant, product: Molecule) -> Molecule {
        self.molecules
            .insert(InnerMolecule::Reaction(reactant, product))
    }
    fn add_product(&mut self, product: Product) -> Molecule {
        self.molecules.insert(InnerMolecule::Product(product))
    }
    fn excite(&mut self, processor: Processor) {
        self.inner.excite(&self.molecules, processor);
    }
    fn step(&mut self) {
        self.inner.step(&self.molecules);
    }
}
#[derive(Debug, Default)]
struct InnerSolution {
    ready: ReadySet,
    reactions: ReactionSet,
}
impl InnerSolution {
    fn new() -> Self {
        InnerSolution {
            reactions: ReactionSet::new(),
            ready: ReadySet::new(),
        }
    }
    fn excite(&mut self, molecules: &MoleculeStore, mut processor: Processor) {
        match molecules.get(processor.molecule) {
            Some(InnerMolecule::Reaction(Reactant::Read(atom, ..), ..)) => {
                self.excite_simple(*atom, processor, true);
            }
            Some(InnerMolecule::Reaction(Reactant::Send(atom, ..), ..)) => {
                self.excite_simple(*atom, processor, false);
            }
            Some(InnerMolecule::Product(Product::Call(c, m))) => {
                c.0();
                processor.molecule = *m;
                self.excite(molecules, processor);
            }
            Some(InnerMolecule::Product(Product::Choice(_))) => unimplemented!(),
            Some(InnerMolecule::Product(Product::Parallel(set))) => {
                for m in set {
                    let p = processor.clone_with(*m);
                    self.excite(molecules, p);
                }
            }
            None => {}
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
    fn step(&mut self, molecules: &MoleculeStore) {
        if let Some(atom) = self.ready.next() {
            let (read, send) = self.reactions.next(atom);
            if self.reactions.is_waiting(atom) {
                self.ready.insert(atom);
            }
            self.react(molecules, read, send);
        }
    }
    fn react(&mut self, molecules: &MoleculeStore, mut read: Processor, send: Processor) {
        match (molecules.get(read.molecule), molecules.get(send.molecule)) {
            (
                Some(InnerMolecule::Reaction(Reactant::Read(_, keys), mr)),
                Some(InnerMolecule::Reaction(Reactant::Send(_, values), ms)),
            ) => {
                read.read(keys, send.send(&values));
                Some((*mr, *ms))
            }
            _ => unreachable!(),
        };
        self.excite_next(molecules, read);
        self.excite_next(molecules, send);
    }
    #[allow(needless_pass_by_value)]
    fn excite_next(&mut self, molecules: &MoleculeStore, mut processor: Processor) {
        match molecules.get(processor.molecule) {
            Some(InnerMolecule::Reaction(_, m))
            | Some(InnerMolecule::Product(Product::Call(_, m))) => {
                processor.molecule = *m;
                self.excite(molecules, processor);
            }
            Some(InnerMolecule::Product(Product::Choice(_set))) => unimplemented!(),
            Some(InnerMolecule::Product(Product::Parallel(set))) => {
                for m in set {
                    let p = processor.clone_with(*m);
                    self.excite(molecules, p);
                }
            }
            None => {}
        };
    }
}
#[derive(Debug, Default)]
struct MoleculeStore {
    data: Vec<InnerMolecule>,
}
impl MoleculeStore {
    fn new() -> Self {
        MoleculeStore {
            data: vec![InnerMolecule::Product(Product::Parallel(Vec::new()))],
        }
    }
    fn get(&self, molecule: Molecule) -> Option<&InnerMolecule> {
        self.data.get(molecule.0)
    }
    fn insert(&mut self, molecule: InnerMolecule) -> Molecule {
        let id = self.data.len();
        self.data.push(molecule);
        Molecule(id)
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
    fn clone_with(&self, molecule: Molecule) -> Self {
        Processor {
            molecule,
            map: self.map.clone(),
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
