extern crate pipl_engine;
use pipl_engine::{Call, Name, Pipl, Prefix, Process, Refs, Sequence};
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
    Process::new_call(call, suffix)
}
fn choice(sequences: Vec<Sequence>) -> Process {
    Process::new_choice(sequences.into_iter().map(|x| Rc::new(x)).collect())
}
fn new_names(names: &[&Name], suffix: Process) -> Process {
    Process::new_names(names.iter().map(|&x| x.clone()).collect(), suffix)
}
fn parallel(sequences: Vec<Sequence>) -> Process {
    Process::new_parallel(sequences.into_iter().map(|x| Rc::new(x)).collect())
}
fn sequence(prefix: Prefix, suffix: Process) -> Process {
    Process::new_sequence(prefix, suffix)
}
struct P {
    channel: Name,
    names: Vec<Name>,
    repeating: bool,
    is_read: bool,
}
impl P {
    fn new(channel: &Name, is_read: bool) -> Self {
        P {
            channel: channel.clone(),
            names: Vec::new(),
            repeating: false,
            is_read: is_read,
        }
    }
    fn read(channel: &Name) -> Self {
        Self::new(channel, true)
    }
    fn send(channel: &Name) -> Self {
        Self::new(channel, false)
    }
    fn names(mut self, names: &[&Name]) -> Self {
        self.names.extend(names.iter().map(|&x| x.clone()));
        self
    }
    fn repeating(mut self) -> Self {
        self.repeating = true;
        self
    }
    fn prefix(&self) -> Prefix {
        let channel = self.channel.clone();
        let names = self.names.clone();
        match (self.repeating, self.is_read) {
            (true, true)   => Prefix::read_many(channel, names),
            (true, false)  => Prefix::send_many(channel, names),
            (false, true)  => Prefix::read(channel, names),
            (false, false) => Prefix::send(channel, names),
        }
    }
    fn logged_sequence(&self, suffix: Process, results: Rc<Results>) -> Process {
        let prefix = self.prefix();
        let key = format!("{}", prefix);
        let suffix = log(key, suffix, results);
        sequence(prefix, suffix)
    }
}
fn make(prefixes: Vec<P>, suffix: Process, results: Rc<Results>) -> Sequence {
    let process = prefixes.into_iter().rev().fold(suffix, |suffix, p| {
        p.logged_sequence(suffix, results.clone())
    });
    match process {
        Process::Sequence(sequence) => Rc::try_unwrap(sequence).unwrap(),
        _ => unreachable!(),
    }
}
fn f(p: &P) -> String {
    format!("{}", p.prefix())
}
fn n(name: u8) -> Name {
    Name::from(vec!(name))
}
fn read(channel: &Name, names: &[&Name]) -> P {
    P::read(channel).names(names)
}
fn read_many(channel: &Name, names: &[&Name]) -> P {
    read(channel, names).repeating()
}
fn send(channel: &Name, names: &[&Name]) -> P {
    P::send(channel).names(names)
}
fn send_many(channel: &Name, names: &[&Name]) -> P {
    send(channel, names).repeating()
}
fn assert_eq_results(left: Rc<Results>, right: Rc<Results>) {
    let mut keys_left = left.0.borrow().keys().cloned().collect::<Vec<_>>();
    let mut keys_right = right.0.borrow().keys().cloned().collect::<Vec<_>>();
    keys_left.sort();
    keys_right.sort();
    assert_eq!(keys_left, keys_right, "results.keys()");
    for ref key in keys_left {
        assert_eq_refs_list(left.get(key), right.get(key), key);
    }
}
fn assert_eq_refs_list(left: Vec<Refs>, right: Vec<Refs>, key: &String) {
    for (i,(l, r)) in left.iter().zip(right.iter()).enumerate() {
        assert_eq_refs(l, r, key, i);
    }
    assert_eq!(left.len(), right.len(), "results[{:?}].len()", key);
}
fn assert_eq_refs(left: &Refs, right: &Refs, key: &String, i: usize) {
    let mut left_keys = left.keys();
    let mut right_keys = right.keys();
    left_keys.sort();
    right_keys.sort();
    assert_eq!(left_keys, right_keys, "results[{:?}][{:?}].keys()", key, i);
    for k in left_keys.iter() {
        assert_eq!(left.get(k).raw(), right.get(k).raw(), "results[{:?}][{:?}][{:?}]", key, i, k);
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
#[test]
fn no_names() {
    // w[].x().() w(a).x[b].()
    let (w, x) = (&n(0x77), &n(0x78));
    let (a, b) = (&n(0x61), &n(0x62));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![read(w, &[]), send(x, &[])], Terminal, actual.clone()));
    pipl.add(make(vec![send(w, &[a]), read(x, &[b])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = Refs::new();
    expected.log(f(&read(w, &[])), refs_empty.clone());
    expected.log(f(&send(x, &[])), refs_empty.clone());
    expected.log(f(&send(w, &[a])), refs_empty.clone());
    expected.log(f(&read(x, &[b])), refs_empty.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
#[test]
fn polyadic() {
    // w[x,y].() w(a,b).()
    let (w, x, y) = (&n(0x77), &n(0x78), &n(0x79));
    let (a, b) = (&n(0x61), &n(0x62));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![read(w, &[x, y])], Terminal, actual.clone()));
    pipl.add(make(vec![send(w, &[a, b])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = Refs::new();
    let refs_read = &mut Refs::new();
    refs_read.set(x.clone(), a.clone());
    refs_read.set(y.clone(), b.clone());
    expected.log(f(&read(w, &[x, y])), refs_read.clone());
    expected.log(f(&send(w, &[a, b])), refs_empty.clone());
    pipl.step();
    assert_eq_results(actual, expected);
}
#[test]
fn new_names_before_parallel() {
    // w[x].[w, x](| w(c).() x[o].() y[p].x(p).x(p) )
    // w(a).w[m].() a[n].() x[o].() y(b).()
    let (w, x, y) = (&n(0x77), &n(0x78), &n(0x79));
    let (a, b, c) = (&n(0x61), &n(0x62), &n(0x63));
    let (m, n, o, p) = (&n(0x6d), &n(0x6e), &n(0x6f), &n(0x70));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(
        vec![read(w, &[x])],
        new_names(&[w, x],
            parallel(vec![
                make(vec![send(w, &[c])], Terminal, actual.clone()),
                make(vec![read(x, &[o])], Terminal, actual.clone()),
                make(vec![read(y, &[p]), send(x, &[p]), send(x, &[p])], Terminal, actual.clone()),
            ]),
        ),
        actual.clone()
    ));
    pipl.add(make(vec![send(w, &[a]), read(w, &[m])], Terminal, actual.clone()));
    pipl.add(make(vec![read(a, &[n])], Terminal, actual.clone()));
    pipl.add(make(vec![send(x, &[o])], Terminal, actual.clone()));
    pipl.add(make(vec![send(y, &[b])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    let refs_yb = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs_wx.clone());
    expected.log(f(&send(w, &[a])), refs_wa.clone());
    pipl.step();
    refs_wx.set(w.clone(), w.dup());
    refs_wx.set(x.clone(), x.dup());
    let refs_wxxo = &mut refs_wx.clone();
    let refs_wxyp = &mut refs_wx.clone();
    refs_wxyp.set(p.clone(), b.clone());
    expected.log(f(&read(y, &[p])), refs_wxyp.clone());
    expected.log(f(&send(y, &[b])), refs_yb.clone());
    pipl.step();
    refs_wxxo.set(o.clone(), b.clone());
    expected.log(f(&read(x, &[o])), refs_wxxo.clone());
    expected.log(f(&send(x, &[p])), refs_wxyp.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
#[test]
fn new_names_before_choice() {
    // w[x].[w, x](+ w(c).() x[o].() y[p].x(p) )
    // w(a).w[m].() a[n].() x[o].() y(b).()
    let (w, x, y) = (&n(0x77), &n(0x78), &n(0x79));
    let (a, b, c) = (&n(0x61), &n(0x62), &n(0x63));
    let (m, n, o, p) = (&n(0x6d), &n(0x6e), &n(0x6f), &n(0x70));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(
        vec![read(w, &[x])],
        new_names(&[w, x],
            choice(vec![
                make(vec![send(w, &[c])], Terminal, actual.clone()),
                make(vec![read(x, &[o])], Terminal, actual.clone()),
                make(vec![read(y, &[p]), send(x, &[p])], Terminal, actual.clone()),
            ]),
        ),
        actual.clone()
    ));
    pipl.add(make(vec![send(w, &[a]), read(w, &[m])], Terminal, actual.clone()));
    pipl.add(make(vec![read(a, &[n])], Terminal, actual.clone()));
    pipl.add(make(vec![send(x, &[o])], Terminal, actual.clone()));
    pipl.add(make(vec![send(y, &[b])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    let refs_yb = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs_wx.clone());
    expected.log(f(&send(w, &[a])), refs_wa.clone());
    pipl.step();
    refs_wx.set(w.clone(), w.dup());
    refs_wx.set(x.clone(), x.dup());
    let refs_wxyp = &mut refs_wx.clone();
    refs_wxyp.set(p.clone(), b.clone());
    expected.log(f(&read(y, &[p])), refs_wxyp.clone());
    expected.log(f(&send(y, &[b])), refs_yb.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
