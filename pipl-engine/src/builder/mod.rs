use ::call::Call;
use ::name::Name;
use ::pipl::Pipl;
use ::prefix::Prefix;
use ::process::Process;
use ::process::sequence::Sequence;
use std::rc::Rc;
pub enum Builder {
    Prefix(PrefixBuilder),
    Parallel(ParallelBuilder),
    Choice(ChoiceBuilder),
    Terminal,
}
pub struct PiplBuilder {
    sequences: Vec<PrefixBuilder>,
}
impl PiplBuilder {
    pub fn new() -> Self {
        PiplBuilder {
            sequences: Vec::new(),
        }
    }
    fn prefix<'a>(&'a mut self, prefix_type: PrefixType, name: Name) -> &'a mut PrefixBuilder {
        let b = PrefixBuilder::new(prefix_type, name);
        self.sequences.push(b);
        self.sequences.last_mut().unwrap()
    }
    /// start building a new sequence with a read prefix
    pub fn read<'a>(&'a mut self, name: Name) -> &'a mut PrefixBuilder {
        self.prefix(PrefixType::Read, name)
    }
    /// start building a new sequence with a send prefix
    pub fn send<'a>(&'a mut self, name: Name) -> &'a mut PrefixBuilder {
        self.prefix(PrefixType::Send, name)
    }
    /// add all sequences to Pipl and remove from builder
    pub fn apply(&mut self, pipl: &mut Pipl) {
        for x in self.sequences.drain(0..) {
            pipl.add(x.build());
        }
    }
}
enum PrefixType {
    Read,
    Send,
}
pub struct PrefixBuilder {
    prefix_type: PrefixType,
    name: Name,
    repeating: bool,
    restricts: Vec<Name>,
    names: Vec<Name>,
    call: Option<Rc<Call>>,
    next: Box<Builder>,
}
impl PrefixBuilder {
    fn new(prefix_type: PrefixType, name: Name) -> Self {
        PrefixBuilder {
            prefix_type: prefix_type,
            name: name,
            repeating: false,
            restricts: Vec::new(),
            names: Vec::new(),
            call: None,
            next: Box::new(Builder::Terminal),
        }
    }
    /// make prefix repeat
    pub fn repeat<'a>(&'a mut self) -> &'a mut Self {
        self.repeating = true;
        self
    }
    /// add names to make unique within prefix
    pub fn restrict<'a>(&'a mut self, names: &[Name]) -> &'a mut Self {
        self.restricts.extend_from_slice(names);
        self
    }
    /// add names to communicate
    pub fn names<'a>(&'a mut self, names: &[Name]) -> &'a mut Self {
        self.names.extend_from_slice(names);
        self
    }
    /// set callback to call between communication and next process
    pub fn call<'a>(&'a mut self, call: Rc<Call>) -> &'a mut Self {
        self.call = Some(call);
        self
    }
    fn prefix<'a>(&'a mut self, prefix_type: PrefixType, name: Name) -> &'a mut PrefixBuilder {
        use std::borrow::BorrowMut;
        self.next = Box::new(Builder::Prefix(PrefixBuilder::new(prefix_type, name)));
        if let &mut Builder::Prefix(ref mut b) = self.next.borrow_mut() {
            b
        }
        else {
            unreachable!()
        }
    }
    /// terminate prefix with a read prefix
    pub fn read<'a>(&'a mut self, name: Name) -> &'a mut PrefixBuilder {
        self.prefix(PrefixType::Read, name)
    }
    /// terminate prefix with a send prefix
    pub fn send<'a>(&'a mut self, name: Name) -> &'a mut PrefixBuilder {
        self.prefix(PrefixType::Send, name)
    }
    /// terminate prefix with a parallel process
    pub fn parallel<'a>(&'a mut self) -> &'a mut ParallelBuilder {
        use std::borrow::BorrowMut;
        self.next = Box::new(Builder::Parallel(ParallelBuilder::new()));
        if let &mut Builder::Parallel(ref mut b) = self.next.borrow_mut() {
            b
        }
        else {
            unreachable!()
        }
    }
    /// terminate prefix with a choice process
    pub fn choice<'a>(&'a mut self) -> &'a mut ChoiceBuilder {
        use std::borrow::BorrowMut;
        self.next = Box::new(Builder::Choice(ChoiceBuilder::new()));
        if let &mut Builder::Choice(ref mut b) = self.next.borrow_mut() {
            b
        }
        else {
            unreachable!()
        }
    }
    fn prepend_new_names((restricts, p): (Vec<Name>, Process)) -> Process {
        if restricts.len() > 0 {
            return Process::new_names(restricts, p)
        }
        p
    }
    fn build(self) -> Sequence {
        let PrefixBuilder {
            prefix_type,
            name,
            repeating,
            restricts,
            names,
            call,
            next,
        } = self;
        let prefix = match (prefix_type, repeating) {
            (PrefixType::Read, false) => Prefix::read(name, names),
            (PrefixType::Send, false) => Prefix::send(name, names),
            (PrefixType::Read, true) => Prefix::read_many(name, names),
            (PrefixType::Send, true) => Prefix::send_many(name, names),
        };
        let mut suffix = match *next {
            Builder::Choice(b)   => Self::prepend_new_names(b.build()),
            Builder::Parallel(b) => Self::prepend_new_names(b.build()),
            Builder::Prefix(b)   => Process::Sequence(Rc::new(b.build())),
            Builder::Terminal    => Process::Terminal,
        };
        if let Some(call) = call {
            suffix = Process::new_call(call, suffix)
        }
        Sequence::new(restricts, prefix, suffix)
    }
}
pub struct ParallelBuilder {
    restricts: Vec<Name>,
    sequences: Vec<PrefixBuilder>,
}
impl ParallelBuilder {
    fn new() -> Self {
        ParallelBuilder {
            restricts: Vec::new(),
            sequences: Vec::new(),
        }
    }
    /// add names to make unique within process
    pub fn restrict<'a>(&'a mut self, names: &[Name]) -> &'a mut Self {
        self.restricts.extend_from_slice(names);
        self
    }
    fn  prefix<'a>(&'a mut self, prefix_type: PrefixType, name: Name) -> &'a mut PrefixBuilder {
        let b = PrefixBuilder::new(prefix_type, name);
        self.sequences.push(b);
        self.sequences.last_mut().unwrap()
    }
    /// start building a new sequence with a read prefix
    pub fn read<'a>(&'a mut self, name: Name) -> &'a mut PrefixBuilder {
        self.prefix(PrefixType::Read, name)
    }
    /// start building a new sequence with a send prefix
    pub fn send<'a>(&'a mut self, name: Name) -> &'a mut PrefixBuilder {
        self.prefix(PrefixType::Send, name)
    }
    fn build(self) -> (Vec<Name>, Process) {
        let ParallelBuilder { restricts, sequences } = self;
        let p = Process::new_parallel(sequences.into_iter().map(|x| Rc::new(x.build())).collect());
        (restricts, p)
    }
}
pub struct ChoiceBuilder {
    restricts: Vec<Name>,
    sequences: Vec<PrefixBuilder>,
}
impl ChoiceBuilder {
    fn new() -> Self {
        ChoiceBuilder {
            restricts: Vec::new(),
            sequences: Vec::new(),
        }
    }
    /// add names to make unique within process
    pub fn restrict<'a>(&'a mut self, names: &[Name]) -> &'a mut Self {
        self.restricts.extend_from_slice(names);
        self
    }
    fn  prefix<'a>(&'a mut self, prefix_type: PrefixType, name: Name) -> &'a mut PrefixBuilder {
        let b = PrefixBuilder::new(prefix_type, name);
        self.sequences.push(b);
        self.sequences.last_mut().unwrap()
    }
    /// start building a new sequence with a read prefix
    pub fn read<'a>(&'a mut self, name: Name) -> &'a mut PrefixBuilder {
        self.prefix(PrefixType::Read, name)
    }
    /// start building a new sequence with a send prefix
    pub fn send<'a>(&'a mut self, name: Name) -> &'a mut PrefixBuilder {
        self.prefix(PrefixType::Send, name)
    }
    fn build(self) -> (Vec<Name>, Process) {
        let ChoiceBuilder { restricts, sequences } = self;
        let p = Process::new_choice(sequences.into_iter().map(|x| Rc::new(x.build())).collect());
        (restricts, p)
    }
}
