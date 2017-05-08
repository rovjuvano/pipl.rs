use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
}
impl OnRead for Read {
    fn read(&self, _mods: &mut Mods, _read: ReadMolecule, _refs: Refs, names: Vec<Name>) {
        self.results.log("read", Name::new(names));
    }
}
#[derive(Debug)]
struct Send {
    names: Vec<Name>,
}
impl OnSend for Send {
    fn send(&self, _mods: &mut Mods, _send: SendMolecule, _refs: Refs) -> Vec<Name> {
        self.names.clone()
    }
}
#[test]
fn simplest_reaction() {
    // w[x] w(a)
    let (w, a) = (&n("w"), &n("a"));
    let actual = &Results::new();
    let wx = read(w, Rc::new(Read { results: actual.clone() }));
    let wa = send(w, Rc::new(Send { names: vec![a.clone()] }));
    let mut pipl = Pipl::new();
    pipl.add(wx);
    pipl.add(wa);
    pipl.step();
    let expected = &Results::new();
    expected.log("read", Name::new(vec![a.clone()]));
    assert_eq_results(actual, expected);
}
