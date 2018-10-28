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
#[derive(Debug)]
pub struct Pipl2 {
    atoms: Vec<String>,
    molecules: Vec<AnyMolecule>,
    waiting_read: BTreeMap<Atom, Vec<Processor>>,
    waiting_send: BTreeMap<Atom, Vec<Processor>>,
    ready: BTreeSet<Atom>,
}
#[derive(Debug)]
struct Processor {
    molecule: Molecule,
    map: BTreeMap<Atom, Atom>,
    //refs
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
#[allow(new_without_default_derive)]
impl Pipl2 {
    pub fn new() -> Self {
        Pipl2 {
            atoms: Vec::new(),
            molecules: vec![AnyMolecule::Terminal],
            waiting_read: BTreeMap::new(),
            waiting_send: BTreeMap::new(),
            ready: BTreeSet::new(),
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
        self.add_molecule(AnyMolecule::Call(call.into(), next))
    }
    pub fn read<'a, I>(&mut self, atom: Atom, atoms: I, next: Molecule) -> Molecule
    where
        I: IntoIterator<Item = &'a Atom>,
    {
        self.add_molecule(AnyMolecule::Simple(SimpleMolecule::Read(
            atom,
            atoms.into_iter().cloned().collect(),
            next,
        )))
    }
    pub fn send<'a, I>(&mut self, atom: Atom, atoms: I, next: Molecule) -> Molecule
    where
        I: IntoIterator<Item = &'a Atom>,
    {
        self.add_molecule(AnyMolecule::Simple(SimpleMolecule::Send(
            atom,
            atoms.into_iter().cloned().collect(),
            next,
        )))
    }
    pub fn repeating_read<'a, I>(&mut self, atom: Atom, atoms: I, next: Molecule) -> Molecule
    where
        I: IntoIterator<Item = &'a Atom>,
    {
        self.add_molecule(AnyMolecule::Repeating(SimpleMolecule::Read(
            atom,
            atoms.into_iter().cloned().collect(),
            next,
        )))
    }
    pub fn repeating_send<'a, I>(&mut self, atom: Atom, atoms: I, next: Molecule) -> Molecule
    where
        I: IntoIterator<Item = &'a Atom>,
    {
        self.add_molecule(AnyMolecule::Repeating(SimpleMolecule::Send(
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
        self.add_molecule(AnyMolecule::Parallel(list))
    }
    pub fn choice<'a, I>(&mut self, molecules: I) -> Molecule
    where
        I: IntoIterator<Item = &'a Molecule>,
    {
        let list = molecules.into_iter().cloned().collect();
        self.add_molecule(AnyMolecule::Choice(list))
    }
    fn add_molecule(&mut self, molecule: AnyMolecule) -> Molecule {
        let id = self.molecules.len();
        self.molecules.push(molecule);
        Molecule(id)
    }
    pub fn excite(&mut self, molecule: Molecule) {
        self.excite_inner(Processor::new(molecule));
    }
    fn excite_inner(&mut self, processor: Processor) {
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
                AnyMolecule::Choice(..) => Excite::None,
                AnyMolecule::Terminal => Excite::None,
            }
        } else {
            Excite::None
        };
        match action {
            Excite::Atom(atom, is_read) => self.excite_simple(atom, processor, is_read),
            Excite::Molecule(m) => self.excite(m),
            Excite::Molecules(list) => for m in list {
                self.excite(m)
            },
            Excite::None => {}
        }
    }
    fn excite_simple(&mut self, atom: Atom, processor: Processor, is_read: bool) {
        let reads = &mut self.waiting_read;
        let sends = &mut self.waiting_send;
        let (this, that) = if is_read {
            (reads, sends)
        } else {
            (sends, reads)
        };
        let atom = processor.get(&atom);
        this.entry(atom).or_insert_with(Vec::new).push(processor);
        use std::collections::btree_map::Entry;
        if let Entry::Occupied(e) = that.entry(atom) {
            if !e.get().is_empty() {
                self.ready.insert(atom);
            }
        }
    }
    pub fn step(&mut self) {
        if let Some(atom) = self.next() {
            let (read, send) = {
                let read = self.waiting_read.get_mut(&atom).unwrap();
                let send = self.waiting_send.get_mut(&atom).unwrap();
                if read.is_empty() || send.is_empty() {
                    self.ready.remove(&atom);
                }
                (read.remove(0), send.remove(0))
            };
            self.react(read, send);
        }
    }
    fn next(&mut self) -> Option<Atom> {
        let maybe = self.ready.iter().next().cloned();
        if let Some(atom) = maybe {
            self.ready.remove(&atom);
            maybe
        } else {
            None
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
            self.excite_inner(processor);
        }
    }
}

pub mod builders;

// #[derive(Copy, Clone)]
// pub struct Name(usize);
// impl fmt::Debug for Name {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Name({})", self.0)
//     }
// }
// #[derive(Debug, Default)]
// pub struct Pipl {
//     processes: Vec<Prefix>,
//     names: Vec<String>,
// }
// impl Pipl {
//     pub fn new() -> Self {
//         Self::default()
//     }
//     pub fn name<S: Into<String>>(&mut self, s: S) -> Name {
//         let id = self.names.len();
//         self.names.push(s.into());
//         Name(id)
//     }
//     fn add_prefix(&mut self, prefix: Prefix) {
//         self.processes.push(prefix);
//     }
// }
// #[derive(Debug)]
// enum Channel {
//     Read(Name),
//     Send(Name),
// }
// #[derive(Debug)]
// pub struct Prefix(Channel, Molecule);
// #[derive(Debug)]
// enum Channel {
//     Read(Name),
//     Send(Name),
// }
// #[derive(Debug, Default)]
// pub struct Tua {
//     steps: Vec<Channel>,
// }
// impl Tua {
//     fn new() -> Self {
//         Self::default()
//     }
//     pub fn read(mut self, name: Name) -> Self {
//         self.steps.push(Channel::Read(name));
//         self
//     }
//     pub fn send(mut self, name: Name) -> Self {
//         self.steps.push(Channel::Send(name));
//         self
//     }
//     pub fn launch(mut self, pipl: &mut Pipl) {
//         let process =
//             self.steps
//                 .drain(..)
//                 .fold(
//                     Suffix::Terminal(Terminal::Sequence),
//                     |suffix, step| match step {
//                         Channel::Read(name) => Suffix::Read(Box::new(Read(name, suffix))),
//                         Channel::Send(name) => Suffix::Send(Box::new(Send(name, suffix))),
//                     },
//                 );
//         match process {
//             Suffix::Read(prefix) => pipl.add_read(*prefix),
//             Suffix::Send(prefix) => pipl.add_send(*prefix),
//             _ => {}
//         };
//     }
// }
// enum Suffix {
//     Read(Box<Read>),
//     Send(Box<Send>),
//     Terminal(Terminal),
// }
// impl fmt::Debug for Suffix {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Suffix::Read(x) => x.fmt(f),
//             Suffix::Send(x) => x.fmt(f),
//             Suffix::Terminal(x) => x.fmt(f),
//         }
//     }
// }
// pub struct Read(Name, Suffix);
// impl Read {
//     pub fn read(self, name: Name) -> Read {
//         Read(name, Suffix::Read(Box::new(self)))
//     }
//     pub fn send(self, name: Name) -> Send {
//         Send(name, Suffix::Read(Box::new(self)))
//     }
//     pub fn choice(self, choice: &mut Choice) {
//         choice.add_read(self)
//     }
//     pub fn launch(self, pipl: &mut Pipl) {
//         pipl.add_read(self);
//     }
//     pub fn parallel(self, parallel: &mut Parallel) {
//         parallel.add_read(self);
//     }
// }
// impl fmt::Debug for Read {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         if let Suffix::Terminal(Terminal::Sequence) = self.1 {
//             write!(f, "Read({:?})", self.0)
//         } else {
//             f.debug_tuple("Read").field(&self.0).field(&self.1).finish()
//         }
//     }
// }
// #[derive(Debug)]
// pub struct Send(Name, Suffix);
// impl Send {
//     pub fn read(self, name: Name) -> Read {
//         Read(name, Suffix::Send(Box::new(self)))
//     }
//     pub fn send(self, name: Name) -> Send {
//         Send(name, Suffix::Send(Box::new(self)))
//     }
//     pub fn choice(self, choice: &mut Choice) {
//         choice.add_send(self)
//     }
//     pub fn launch(self, pipl: &mut Pipl) {
//         pipl.add_send(self);
//     }
//     pub fn parallel(self, parallel: &mut Parallel) {
//         parallel.add_send(self)
//     }
// }
// // impl fmt::Debug for Send {
// //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// //         if let Suffix::Terminal(Terminal::Sequence) = self.1 {
// //             write!(f, "Send({:?})", self.0)
// //         } else {
// //             f.debug_tuple("Send").field(&self.0).field(&self.1).finish()
// //         }
// //     }
// // }
// #[derive(Debug, Default)]
// pub struct Choice {
//     processes: Vec<Prefix>,
// }
// impl Choice {
//     pub fn new() -> Self {
//         Self::default()
//     }
//     pub fn prefix(self) -> Terminal {
//         Terminal::Choice(self)
//     }
//     fn add_read(&mut self, read: Read) {
//         self.processes.push(Prefix::Read(read));
//     }
//     fn add_send(&mut self, send: Send) {
//         self.processes.push(Prefix::Send(send));
//     }
// }
// #[derive(Debug, Default)]
// pub struct Parallel {
//     processes: Vec<Prefix>,
// }
// impl Parallel {
//     pub fn new() -> Self {
//         Self::default()
//     }
//     pub fn prefix(self) -> Terminal {
//         Terminal::Parallel(self)
//     }
//     fn add_read(&mut self, read: Read) {
//         self.processes.push(Prefix::Read(read));
//     }
//     fn add_send(&mut self, send: Send) {
//         self.processes.push(Prefix::Send(send));
//     }
// }
// pub enum Terminal {
//     Choice(Choice),
//     Parallel(Parallel),
//     Sequence,
// }
// impl Terminal {
//     pub fn send(self, name: Name) -> Send {
//         Send(name, Suffix::Terminal(self))
//     }
//     pub fn read(self, name: Name) -> Read {
//         Read(name, Suffix::Terminal(self))
//     }
// }
// impl fmt::Debug for Terminal {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Terminal::Choice(x) => x.fmt(f),
//             Terminal::Parallel(x) => x.fmt(f),
//             Terminal::Sequence => f.write_str("Terminal"),
//         }
//     }
// }
