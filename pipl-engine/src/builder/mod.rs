use crate::call::Call;
use crate::name::Name;
use crate::pipl::Pipl;
use crate::prefix::Action;
use crate::prefix::Prefix;
use crate::prefix::PrefixDirection;
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
    fn prefix<'a>(&'a mut self, name: &Name, direction: PrefixDirection) -> &'a mut PrefixBuilder {
        let b = PrefixBuilder::new(name.clone(), direction);
        self.sequences.push(b);
        self.sequences.last_mut().unwrap()
    }
    /// start building a new sequence with a read prefix
    pub fn read<'a>(&'a mut self, name: &Name) -> &'a mut PrefixBuilder {
        self.prefix(name, PrefixDirection::Read)
    }
    /// start building a new sequence with a send prefix
    pub fn send<'a>(&'a mut self, name: &Name) -> &'a mut PrefixBuilder {
        self.prefix(name, PrefixDirection::Send)
    }
    /// add all sequences to Pipl and remove from builder
    pub fn apply(&mut self, pipl: &mut Pipl) {
        for x in self.sequences.drain(0..) {
            pipl.add(x.build());
        }
    }
}
pub struct PrefixBuilder {
    name: Name,
    direction: PrefixDirection,
    repeating: bool,
    restricts: Vec<Name>,
    names: Vec<Name>,
    call: Option<Box<dyn Call>>,
    next: Box<Builder>,
}
impl PrefixBuilder {
    fn new(name: Name, direction: PrefixDirection) -> Self {
        PrefixBuilder {
            name: name,
            direction: direction,
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
    pub fn restrict<'a>(&'a mut self, names: &[&Name]) -> &'a mut Self {
        self.restricts.extend(names.iter().map(|&x| x.clone()));
        self
    }
    /// add names to communicate
    pub fn names<'a>(&'a mut self, names: &[&Name]) -> &'a mut Self {
        self.names.extend(names.iter().map(|&x| x.clone()));
        self
    }
    /// set callback to call between communication and next process
    pub fn call<'a, T: Call + 'static>(&'a mut self, call: T) -> &'a mut Self {
        self.call = Some(Box::new(call));
        self
    }
    fn prefix<'a>(&'a mut self, name: &Name, direction: PrefixDirection) -> &'a mut PrefixBuilder {
        use std::borrow::BorrowMut;
        self.next = Box::new(Builder::Prefix(PrefixBuilder::new(name.clone(), direction)));
        if let &mut Builder::Prefix(ref mut b) = self.next.borrow_mut() {
            b
        } else {
            unreachable!()
        }
    }
    /// terminate prefix with a read prefix
    pub fn read<'a>(&'a mut self, name: &Name) -> &'a mut PrefixBuilder {
        self.prefix(name, PrefixDirection::Read)
    }
    /// terminate prefix with a send prefix
    pub fn send<'a>(&'a mut self, name: &Name) -> &'a mut PrefixBuilder {
        self.prefix(name, PrefixDirection::Send)
    }
    /// terminate prefix with a parallel process
    pub fn parallel<'a>(&'a mut self) -> &'a mut ParallelBuilder {
        use std::borrow::BorrowMut;
        self.next = Box::new(Builder::Parallel(ParallelBuilder::new()));
        if let &mut Builder::Parallel(ref mut b) = self.next.borrow_mut() {
            b
        } else {
            unreachable!()
        }
    }
    /// terminate prefix with a choice process
    pub fn choice<'a>(&'a mut self) -> &'a mut ChoiceBuilder {
        use std::borrow::BorrowMut;
        self.next = Box::new(Builder::Choice(ChoiceBuilder::new()));
        if let &mut Builder::Choice(ref mut b) = self.next.borrow_mut() {
            b
        } else {
            unreachable!()
        }
    }
    fn build(self) -> Prefix {
        let PrefixBuilder {
            name,
            direction,
            repeating,
            restricts,
            names,
            call,
            next,
        } = self;
        let mut actions = Vec::new();
        if repeating {
            actions.push(Action::Repeat);
        }
        if restricts.len() > 0 {
            actions.push(Action::Restrict(restricts));
        }
        if names.len() > 0 {
            actions.push(Action::Communicate(names));
        }
        if let Some(call) = call {
            actions.push(Action::Call(call));
        }
        match *next {
            Builder::Choice(b) => {
                let (restricts, sequences) = b.build();
                if restricts.len() > 0 {
                    actions.push(Action::Restrict(restricts));
                }
                actions.push(Action::Choice(sequences));
            }
            Builder::Parallel(b) => {
                let (restricts, sequences) = b.build();
                if restricts.len() > 0 {
                    actions.push(Action::Restrict(restricts));
                }
                actions.push(Action::Parallel(sequences));
            }
            Builder::Prefix(b) => actions.push(Action::Prefix(b.build())),
            Builder::Terminal => {}
        };
        Prefix::new(name, direction, actions)
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
    pub fn restrict<'a>(&'a mut self, names: &[&Name]) -> &'a mut Self {
        self.restricts.extend(names.iter().map(|&x| x.clone()));
        self
    }
    fn prefix<'a>(&'a mut self, name: &Name, direction: PrefixDirection) -> &'a mut PrefixBuilder {
        let b = PrefixBuilder::new(name.clone(), direction);
        self.sequences.push(b);
        self.sequences.last_mut().unwrap()
    }
    /// start building a new sequence with a read prefix
    pub fn read<'a>(&'a mut self, name: &Name) -> &'a mut PrefixBuilder {
        self.prefix(name, PrefixDirection::Read)
    }
    /// start building a new sequence with a send prefix
    pub fn send<'a>(&'a mut self, name: &Name) -> &'a mut PrefixBuilder {
        self.prefix(name, PrefixDirection::Send)
    }
    fn build(self) -> (Vec<Name>, Vec<Prefix>) {
        let ParallelBuilder {
            restricts,
            sequences,
        } = self;
        let p = sequences.into_iter().map(|x| x.build()).collect();
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
    pub fn restrict<'a>(&'a mut self, names: &[&Name]) -> &'a mut Self {
        self.restricts.extend(names.iter().map(|&x| x.clone()));
        self
    }
    fn prefix<'a>(&'a mut self, name: &Name, direction: PrefixDirection) -> &'a mut PrefixBuilder {
        let b = PrefixBuilder::new(name.clone(), direction);
        self.sequences.push(b);
        self.sequences.last_mut().unwrap()
    }
    /// start building a new sequence with a read prefix
    pub fn read<'a>(&'a mut self, name: &Name) -> &'a mut PrefixBuilder {
        self.prefix(name, PrefixDirection::Read)
    }
    /// start building a new sequence with a send prefix
    pub fn send<'a>(&'a mut self, name: &Name) -> &'a mut PrefixBuilder {
        self.prefix(name, PrefixDirection::Send)
    }
    fn build(self) -> (Vec<Name>, Vec<Prefix>) {
        let ChoiceBuilder {
            restricts,
            sequences,
        } = self;
        let p = sequences.into_iter().map(|x| x.build()).collect();
        (restricts, p)
    }
}
