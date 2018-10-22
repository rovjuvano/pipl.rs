use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
}
impl OnRead<N> for Read {
    fn read(&self, _mods: &mut Mods<N>, _read: ReadMolecule<N>, _refs: Refs<N>, names: Vec<Name<N>>) {
        self.results.log("read", N::vec(names));
    }
}
#[derive(Debug)]
struct Send {
    names: Vec<Name<N>>,
}
impl OnSend<N> for Send {
    fn send(&self, _mods: &mut Mods<N>, _send: SendMolecule<N>, _refs: Refs<N>) -> Vec<Name<N>> {
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
    expected.log("read", N::vec(vec![a.clone()]));
    assert_eq_results(actual, expected);
}
