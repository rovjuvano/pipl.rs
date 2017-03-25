extern crate pipl_engine;
use pipl_engine::{Call, CallProcess, Name, Pipl, Prefix, Process, Refs, Sequence};
use pipl_engine::Process::Terminal;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug, Eq, PartialEq)]
struct Results(RefCell<HashMap<String, Vec<Refs>>>);
impl Results {
    fn new() -> Self {
        Results(RefCell::new(HashMap::new()))
    }
    fn log<K: Into<String>>(&self, key: K, refs: Refs) {
        self.0.borrow_mut()
            .entry(key.into())
            .or_insert(Vec::new())
            .push(refs);
    }
    fn get(&self, key: &str) -> Vec<Refs> {
        self.0.borrow()
            .get(key)
            .or(Some(&Vec::with_capacity(0)))
            .unwrap().clone()
    }
}
#[derive(Debug)]
struct ResultsCall {
    key: String,
    results: Rc<Results>,
}
impl ResultsCall {
    fn new<K: Into<String>>(key: K, results: Rc<Results>) -> Self {
        ResultsCall {
            key: key.into(),
            results: results,
        }
    }
}
impl Call for ResultsCall {
    fn call(&self, refs: Refs) -> Refs {
        self.results.log(self.key.clone(), refs.clone());
        refs
    }
}
fn log<K: Into<String>>(key: K, suffix: Process, results: Rc<Results>) -> Process {
    call(Rc::new(ResultsCall::new(key, results.clone())), suffix)
}
fn call(call: Rc<Call>, suffix: Process) -> Process {
    Process::Call(Rc::new(CallProcess::new(call, suffix)))
}
fn sequence(prefix: Prefix, suffix: Process) -> Process {
    Process::Sequence(Rc::new(Sequence::new(prefix, suffix)))
}
fn make(prefixes: Vec<Prefix>, suffix: Process, results: Rc<Results>) -> Sequence {
    let process = prefixes.into_iter().rev().fold(suffix, |suffix, prefix| {
        let key = f(&prefix);
        sequence(prefix, log(key, suffix, results.clone()))
    });
    match process {
        Process::Sequence(sequence) => Rc::try_unwrap(sequence).unwrap(),
        _ => unreachable!(),
    }
}
fn f(prefix: &Prefix) -> String {
    format!("{}", &prefix)
}
fn n(name: u8) -> Name {
    Name::from(vec!(name))
}
fn read(channel: &Name, names: &[&Name]) -> Prefix {
    Prefix::read(channel.clone(), names.iter().map(|&x| x.clone()).collect(), false)
}
fn send(channel: &Name, names: &[&Name]) -> Prefix {
    Prefix::send(channel.clone(), names.iter().map(|&x| x.clone()).collect(), false)
}
fn assert_eq_results(left: Rc<Results>, right: Rc<Results>) {
    use std::iter::Extend;
    let mut keys = Vec::new();
    keys.extend(left.0.borrow().keys().cloned());
    keys.extend(right.0.borrow().keys().cloned());
    keys.sort();
    keys.dedup();
    for ref key in keys {
        assert_eq!(left.get(key), right.get(key), "{:?}", key);
    }
}
#[test]
fn wx_wy() {
    // w[x] w(a)
    let (w, x) = (&n(0x77), &n(0x78));
    let a = &n(0x61);
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![read(w, &[x])], Terminal, actual.clone()));
    pipl.add(make(vec![send(w, &[a])], Terminal, actual.clone()));
    pipl.step();
    let expected = Rc::new(Results::new());
    let refs = &mut Refs::new();
    expected.log(f(&send(w, &[a])), refs.clone());
    refs.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs.clone());
    assert_eq_results(actual, expected);
}
