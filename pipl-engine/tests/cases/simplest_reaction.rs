use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
}
impl OnRead<N> for Read {
    fn read(&self, _mods: &mut Mods<N>, _read: ReadMolecule<N>, _refs: Refs, names: Vec<Name>) {
        self.results.log("read", N::Vec(names));
    }
}
#[derive(Debug)]
struct Send {
    names: Vec<Name>,
}
impl OnSend<N> for Send {
    fn send(&self, _mods: &mut Mods<N>, _send: SendMolecule<N>, _refs: Refs) -> Vec<Name> {
        self.names.clone()
    }
}
#[test]
fn simplest_reaction() {
    // w[x] w(a)
    let mut pipl = Pipl::new();
    names!(|pipl| { w a });
    // let (w, a) = (n!("w"), n!("a"));
    let actual = &Results::new();
    let wx = read(w, Rc::new(Read { results: actual.clone() }));
    let wa = send(w, Rc::new(Send { names: vec![a.clone()] }));
    pipl.add(wx);
    pipl.add(wa);
    pipl.step();
    let expected = &Results::new();
    expected.log("read", N::Vec(vec![a.clone()]));
    assert_eq_results(actual, expected);
}
