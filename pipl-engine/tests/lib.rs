extern crate pipl_engine;
use pipl_engine::{Call, CallProcess, ChoiceProcess, Name, ParallelProcess, Pipl, Prefix, Process, Refs, Sequence};
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
fn choice(sequences: Vec<Sequence>) -> Process {
    Process::Choice(Rc::new(ChoiceProcess::new(
        sequences.into_iter().map(|x| Rc::new(x)).collect()
    )))
}
fn parallel(sequences: Vec<Sequence>) -> Process {
    Process::Parallel(Rc::new(ParallelProcess::new(
        sequences.into_iter().map(|x| Rc::new(x)).collect()
    )))
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
fn _read(channel: &Name, names: &[&Name], repeating: bool) -> Prefix {
    Prefix::read(channel.clone(), names.iter().map(|&x| x.clone()).collect(), repeating)
}
fn read(channel: &Name, names: &[&Name]) -> Prefix {
    _read(channel, names, false)
}
fn read_many(channel: &Name, names: &[&Name]) -> Prefix {
    _read(channel, names, true)
}
fn _send(channel: &Name, names: &[&Name], repeating: bool) -> Prefix {
    Prefix::send(channel.clone(), names.iter().map(|&x| x.clone()).collect(), repeating)
}
fn send(channel: &Name, names: &[&Name]) -> Prefix {
    _send(channel, names, false)
}
fn send_many(channel: &Name, names: &[&Name]) -> Prefix {
    _send(channel, names, true)
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
fn simplest_reaction() {
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
#[test]
fn multi_step_reaction() {
    // w[x].w[y] w(a).w(b)
    let (w, x, y) = (&n(0x77), &n(0x78), &n(0x79));
    let (a, b) = (&n(0x61), &n(0x62));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![read(w, &[x]), read(w, &[y])], Terminal, actual.clone()));
    pipl.add(make(vec![send(w, &[a]), send(w, &[b])], Terminal, actual.clone()));
    pipl.step();
    pipl.step();
    let expected = Rc::new(Results::new());
    let refs = &mut Refs::new();
    expected.log(f(&send(w, &[a])), refs.clone());
    expected.log(f(&send(w, &[b])), refs.clone());
    refs.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs.clone());
    refs.set(y.clone(), b.clone());
    expected.log(f(&read(w, &[y])), refs.clone());
    assert_eq_results(actual, expected);
}
#[test]
fn simplest_mobility() {
    // w(x).x[y].() w[z].z(z).()
    let (w, x, y, z) = (&n(0x77), &n(0x78), &n(0x79), &n(0x80));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![send(w, &[x]), read(x, &[y])], Terminal, actual.clone()));
    pipl.add(make(vec![read(w, &[z]), send(z, &[z])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wz = &mut Refs::new();
    refs_wz.set(z.clone(), x.clone());
    expected.log(f(&send(w, &[x])), refs_wx.clone());
    expected.log(f(&read(w, &[z])), refs_wz.clone());
    pipl.step();
    assert_eq_results(actual.clone(), expected.clone());
    refs_wx.set(y.clone(), x.clone());
    expected.log(f(&read(x, &[y])), refs_wx.clone());
    expected.log(f(&send(z, &[z])), refs_wz.clone());
    pipl.step();
    assert_eq_results(actual, expected);
}
#[test]
fn repeating_read_prefix() {
    // w(a).a(c).w(b).b(c).a(d).b(e).() !w[x].!x[y].()
    let (w, x, y) = (&n(0x77), &n(0x78), &n(0x79));
    let (a, b, c, d, e) = (&n(0x61), &n(0x62), &n(0x63), &n(0x64), &n(0x65));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![
            send(w, &[a]), send(a, &[c]),
            send(w, &[b]), send(b, &[c]),
            send(a, &[d]), send(b, &[e]),
        ],
        Terminal, actual.clone())
    );
    pipl.add(make(vec![read_many(w, &[x]), read_many(x, &[y])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = &mut Refs::new();
    let refs_wx1 = &mut Refs::new();
    let refs_wx2 = &mut Refs::new();
    // w(a).a(c) !w[x].!x[y]
    expected.log(f(&send(w, &[a])), refs_empty.clone());
    expected.log(f(&send(a, &[c])), refs_empty.clone());
    refs_wx1.set(x.clone(), a.clone());
    expected.log(f(&read_many(w, &[x])), refs_wx1.clone());
    refs_wx1.set(y.clone(), c.clone());
    expected.log(f(&read_many(x, &[y])), refs_wx1.clone());
    pipl.step();
    pipl.step();
    // w(b).b(c) !w[x].!x[y]
    expected.log(f(&send(w, &[b])), refs_empty.clone());
    expected.log(f(&send(b, &[c])), refs_empty.clone());
    refs_wx2.set(x.clone(), b.clone());
    expected.log(f(&read_many(w, &[x])), refs_wx2.clone());
    refs_wx2.set(y.clone(), c.clone());
    expected.log(f(&read_many(x, &[y])), refs_wx2.clone());
    pipl.step();
    pipl.step();
    // a(d).b(e) !x[y].()
    expected.log(f(&send(a, &[d])), refs_empty.clone());
    expected.log(f(&send(b, &[e])), refs_empty.clone());
    refs_wx1.set(y.clone(), d.clone());
    expected.log(f(&read_many(x, &[y])), refs_wx1.clone());
    refs_wx2.set(y.clone(), e.clone());
    expected.log(f(&read_many(x, &[y])), refs_wx2.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
#[test]
fn repeating_send_prefix() {
    // w[a].a[c].w[b].b[c].a[d].b[e].() !w(x).!x(y).()
    let (w, x, y) = (&n(0x77), &n(0x78), &n(0x79));
    let (a, b, c, d, e) = (&n(0x61), &n(0x62), &n(0x63), &n(0x64), &n(0x65));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![
            read(w, &[a]), read(a, &[c]),
            read(w, &[b]), read(b, &[c]),
            read(a, &[d]), read(b, &[e]),
        ],
        Terminal, actual.clone())
    );
    pipl.add(make(vec![send_many(w, &[x]), send_many(x, &[y])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = Refs::new();
    let refs_wa = &mut Refs::new();
    // w[a].a[c] !w(x).!x(y)
    refs_wa.set(a.clone(), x.clone());
    expected.log(f(&read(w, &[a])), refs_wa.clone());
    refs_wa.set(c.clone(), y.clone());
    expected.log(f(&read(a, &[c])), refs_wa.clone());
    expected.log(f(&send_many(w, &[x])), refs_empty.clone());
    expected.log(f(&send_many(x, &[y])), refs_empty.clone());
    pipl.step();
    pipl.step();
    // w[b].b[c] !w(x).!x(y)
    refs_wa.set(b.clone(), x.clone());
    expected.log(f(&read(w, &[b])), refs_wa.clone());
    refs_wa.set(c.clone(), y.clone());
    expected.log(f(&read(b, &[c])), refs_wa.clone());
    expected.log(f(&send_many(w, &[x])), refs_empty.clone());
    expected.log(f(&send_many(x, &[y])), refs_empty.clone());
    pipl.step();
    pipl.step();
    // a[d].b[e] !x(y).()
    refs_wa.set(d.clone(), y.clone());
    expected.log(f(&read(a, &[d])), refs_wa.clone());
    refs_wa.set(e.clone(), y.clone());
    expected.log(f(&read(b, &[e])), refs_wa.clone());
    expected.log(f(&send_many(x, &[y])), refs_empty.clone());
    expected.log(f(&send_many(x, &[y])), refs_empty.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
#[test]
fn terminate_parallel() {
    // w[x].(| x[y].() y[z].y[z].() ) w(a).a(b).y(c).() b(d).()
    let (w, x, y, z) = (&n(0x77), &n(0x78), &n(0x79), &n(0x80));
    let (a, b, c, d) = (&n(0x61), &n(0x62), &n(0x63), &n(0x64));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(
        vec![read(w, &[x])],
        parallel(vec![
            make(vec![read(x, &[y])], Terminal, actual.clone()),
            make(vec![read(y, &[z]), read(y, &[z])], Terminal, actual.clone()),
        ]),
        actual.clone()
    ));
    pipl.add(make(vec![send(w, &[a]), send(a, &[b]), send(y, &[c])], Terminal, actual.clone()));
    pipl.add(make(vec![send(b, &[d])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = Refs::new();
    let refs_wx = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs_wx.clone());
    let refs_wxxy = &mut refs_wx.clone();
    let refs_wxyz = &mut refs_wx.clone();
    refs_wxxy.set(y.clone(), b.clone());
    refs_wxyz.set(z.clone(), c.clone());
    expected.log(f(&read(x, &[y])), refs_wxxy.clone());
    expected.log(f(&read(y, &[z])), refs_wxyz.clone());
    expected.log(f(&send(w, &[a])), refs_empty.clone());
    expected.log(f(&send(a, &[b])), refs_empty.clone());
    expected.log(f(&send(y, &[c])), refs_empty.clone());
    pipl.step();
    pipl.step();
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
#[test]
fn terminate_choice() {
    // w[x].(+ x[y].() y[z].y[z].() ) w(a).a(b).y(c).() b(d).()
    let (w, x, y, z) = (&n(0x77), &n(0x78), &n(0x79), &n(0x80));
    let (a, b, c, d) = (&n(0x61), &n(0x62), &n(0x63), &n(0x64));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(
        vec![read(w, &[x])],
        choice(vec![
            make(vec![read(x, &[y])], Terminal, actual.clone()),
            make(vec![read(y, &[z]), read(y, &[z])], Terminal, actual.clone()),
        ]),
        actual.clone()
    ));
    pipl.add(make(vec![send(w, &[a]), send(a, &[b]), send(y, &[c])], Terminal, actual.clone()));
    pipl.add(make(vec![send(b, &[d])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = Refs::new();
    let refs_wx = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs_wx.clone());
    let refs_wxxy = &mut refs_wx.clone();
    refs_wxxy.set(y.clone(), b.clone());
    expected.log(f(&read(x, &[y])), refs_wxxy.clone());
    expected.log(f(&send(w, &[a])), refs_empty.clone());
    expected.log(f(&send(a, &[b])), refs_empty.clone());
    pipl.step();
    pipl.step();
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
