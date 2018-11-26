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
impl<'a> Into<Atom> for &'a Atom {
    fn into(self) -> Atom {
        *self
    }
}
impl<'a: 'b, 'b> Into<Atom> for &'b &'a Atom {
    fn into(self) -> Atom {
        **self
    }
}
#[derive(Copy, Clone)]
pub struct Reactant(usize);
impl fmt::Debug for Reactant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reactant({})", self.0)
    }
}
impl Reactant {
    pub fn reaction(self, product: Product) -> Reaction {
        Reaction(self, product)
    }
}
impl<'a> Into<Reactant> for &'a Reactant {
    fn into(self) -> Reactant {
        *self
    }
}
impl<'a: 'b, 'b> Into<Reactant> for &'b &'a Reactant {
    fn into(self) -> Reactant {
        **self
    }
}
#[derive(Copy, Clone)]
pub struct Product(usize);
impl fmt::Debug for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Product({})", self.0)
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Reaction(Reactant, Product);
impl<'a> Into<Reaction> for &'a Reaction {
    fn into(self) -> Reaction {
        *self
    }
}
impl<'a: 'b, 'b> Into<Reaction> for &'b &'a Reaction {
    fn into(self) -> Reaction {
        **self
    }
}
#[derive(Debug, Clone)]
enum InnerReactant {
    Read(ReactantData),
    Send(ReactantData),
}
#[derive(Debug, Clone)]
struct ReactantData {
    atom: Atom,
    names: Vec<Atom>,
}
impl InnerReactant {
    pub fn read(atom: Atom, names: Vec<Atom>) -> Self {
        InnerReactant::Read(ReactantData { atom, names })
    }
    pub fn send(atom: Atom, names: Vec<Atom>) -> Self {
        InnerReactant::Send(ReactantData { atom, names })
    }
    pub fn atom(&self) -> &Atom {
        use InnerReactant::*;
        match self {
            Read(data) | Send(data) => &data.atom,
        }
    }
    pub fn is_read(&self) -> bool {
        use InnerReactant::*;
        match self {
            Read(_) => true,
            Send(_) => false,
        }
    }
    pub fn names(&self) -> &[Atom] {
        use InnerReactant::*;
        match self {
            Read(data) | Send(data) => &data.names,
        }
    }
}
#[derive(Debug)]
enum InnerProduct {
    Call(Call, Product),
    Choice(Vec<Reaction>),
    Parallel(Vec<Reaction>),
    Sequence(Reaction),
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
    pub fn terminal(&mut self) -> Product {
        Product(0)
    }
    pub fn reaction(&mut self, reactant: Reactant, product: Product) -> Reaction {
        Reaction(reactant, product)
    }
    pub fn read<I>(&mut self, atom: Atom, atoms: I) -> Reactant
    where
        I: IntoIterator,
        I::Item: Into<Atom>,
    {
        let atoms = atoms.into_iter().map(|x| x.into()).collect();
        self.solution.add_reactant(InnerReactant::read(atom, atoms))
    }
    pub fn send<I>(&mut self, atom: Atom, atoms: I) -> Reactant
    where
        I: IntoIterator,
        I::Item: Into<Atom>,
    {
        let atoms = atoms.into_iter().map(|x| x.into()).collect();
        self.solution.add_reactant(InnerReactant::send(atom, atoms))
    }
    pub fn sequence(&mut self, reaction: Reaction) -> Product {
        self.solution.add_product(InnerProduct::Sequence(reaction))
    }
    pub fn parallel<I>(&mut self, reactions: I) -> Product
    where
        I: IntoIterator,
        I::Item: Into<Reaction>,
    {
        let list = reactions.into_iter().map(|x| x.into()).collect();
        self.solution.add_product(InnerProduct::Parallel(list))
    }
    pub fn choice<I>(&mut self, reactions: I) -> Product
    where
        I: IntoIterator,
        I::Item: Into<Reaction>,
    {
        let list = reactions.into_iter().map(|x| x.into()).collect();
        self.solution.add_product(InnerProduct::Choice(list))
    }
    pub fn call<I: Into<Call>>(&mut self, call: I, next: Product) -> Product {
        self.solution
            .add_product(InnerProduct::Call(call.into(), next))
    }
    pub fn chain<I>(&mut self, reactants: I, product: Product) -> Reaction
    where
        I: IntoIterator,
        I::Item: Into<Reactant>,
        I::IntoIter: DoubleEndedIterator,
    {
        let mut iter = reactants.into_iter();
        if let Some(x) = iter.next_back() {
            let mut reaction = Reaction(x.into(), product);
            while let Some(x) = iter.next_back() {
                let p = self.sequence(reaction);
                reaction = Reaction(x.into(), p);
            }
            return reaction;
        }
        unreachable!();
    }
    pub fn excite(&mut self, reaction: Reaction) {
        self.solution.excite(Processor::new(reaction));
    }
    pub fn step(&mut self) {
        self.solution.step();
    }
}
#[derive(Debug, Default)]
struct Solution {
    products: ProductStore,
    reactants: ReactantStore,
    reaction_set: ReactionSet,
    ready: ReadySet,
}
impl Solution {
    fn new() -> Self {
        Solution {
            products: ProductStore::new(),
            reactants: ReactantStore::new(),
            reaction_set: ReactionSet::new(),
            ready: ReadySet::new(),
        }
    }
    fn add_product(&mut self, product: InnerProduct) -> Product {
        self.products.insert(product)
    }
    fn add_reactant(&mut self, reactant: InnerReactant) -> Reactant {
        self.reactants.insert(reactant)
    }
    fn reactor(&mut self) -> Reactor {
        Reactor::new(
            &self.products,
            &self.reactants,
            &mut self.reaction_set,
            &mut self.ready,
        )
    }
    fn excite(&mut self, processor: Processor) {
        self.reactor().excite(processor);
    }
    fn step(&mut self) {
        self.reactor().step();
    }
}
#[derive(Debug)]
struct Reactor<'a> {
    products: &'a ProductStore,
    reactants: &'a ReactantStore,
    reaction_set: &'a mut ReactionSet,
    ready: &'a mut ReadySet,
}
impl<'a> Reactor<'a> {
    fn new(
        products: &'a ProductStore,
        reactants: &'a ReactantStore,
        reaction_set: &'a mut ReactionSet,
        ready: &'a mut ReadySet,
    ) -> Self {
        Reactor {
            products,
            reactants,
            reaction_set,
            ready,
        }
    }
    fn excite(&mut self, processor: Processor) {
        if let Some(reactant) = self.reactants.get(processor.reaction.0) {
            let atom = processor.get(reactant.atom());
            if reactant.is_read() {
                self.reaction_set.insert_read(atom, processor);
            } else {
                self.reaction_set.insert_send(atom, processor);
            }
            if self.reaction_set.is_waiting(atom) {
                self.ready.insert(atom);
            }
        }
    }
    fn step(&mut self) {
        if let Some(atom) = self.ready.next() {
            let (read, send) = self.reaction_set.next(atom);
            if self.reaction_set.is_waiting(atom) {
                self.ready.insert(atom);
            }
            self.react(read, send);
        }
    }
    fn react(&mut self, mut read: Processor, send: Processor) {
        match (
            self.reactants.get(read.reaction.0),
            self.reactants.get(send.reaction.0),
        ) {
            (Some(r), Some(s)) => {
                read.read(r.names(), send.send(s.names()));
            }
            _ => unreachable!(),
        };
        self.excite_next(send);
        self.excite_next(read);
    }
    fn excite_next(&mut self, processor: Processor) {
        self.excite_next_product(processor.reaction.1, processor);
    }
    fn excite_next_product(&mut self, product: Product, mut processor: Processor) {
        match self.products.get(product) {
            Some(InnerProduct::Sequence(r)) => {
                processor.reaction = *r;
                self.excite(processor);
            }
            Some(InnerProduct::Call(c, p)) => {
                (c.0)();
                self.excite_next_product(*p, processor);
            }
            Some(InnerProduct::Choice(_set)) => unimplemented!(),
            Some(InnerProduct::Parallel(set)) => {
                for r in set {
                    let p = processor.clone_with(*r);
                    self.excite(p);
                }
            }
            None => {}
        };
    }
}
#[derive(Debug, Default)]
struct ReactantStore {
    data: Vec<InnerReactant>,
}
impl ReactantStore {
    fn new() -> Self {
        ReactantStore { data: Vec::new() }
    }
    fn get(&self, reactant: Reactant) -> Option<&InnerReactant> {
        self.data.get(reactant.0)
    }
    fn insert(&mut self, reactant: InnerReactant) -> Reactant {
        let id = self.data.len();
        self.data.push(reactant);
        Reactant(id)
    }
}

#[derive(Debug, Default)]
struct ProductStore {
    data: Vec<InnerProduct>,
}
impl ProductStore {
    fn new() -> Self {
        ProductStore {
            data: vec![InnerProduct::Parallel(Vec::new())],
        }
    }
    fn get(&self, product: Product) -> Option<&InnerProduct> {
        self.data.get(product.0)
    }
    fn insert(&mut self, product: InnerProduct) -> Product {
        let id = self.data.len();
        self.data.push(product);
        Product(id)
    }
}
#[derive(Debug)]
struct Processor {
    reaction: Reaction,
    map: BTreeMap<Atom, Atom>,
}
impl Processor {
    fn new(reaction: Reaction) -> Self {
        Processor {
            reaction,
            map: BTreeMap::new(),
        }
    }
    fn clone_with(&self, reaction: Reaction) -> Self {
        Processor {
            reaction,
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
    fn _entry(&mut self, atom: Atom) -> &mut (Vec<Processor>, Vec<Processor>) {
        self.set
            .entry(atom)
            .or_insert_with(|| (Vec::new(), Vec::new()))
    }
    fn insert_read(&mut self, atom: Atom, processor: Processor) {
        self._entry(atom).0.push(processor);
    }
    fn insert_send(&mut self, atom: Atom, processor: Processor) {
        self._entry(atom).1.push(processor);
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
