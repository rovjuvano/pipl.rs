use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
    names: Vec<Name<N>>,
}
impl Read {
    fn new(results: &Rc<Results>, names: &[&Name<N>]) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: unslice(names),
        })
    }
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
impl Send {
    fn new(names: &[&Name<N>]) -> Rc<Self> {
        Rc::new(Send {
            names: unslice(names),
        })
    }
}
impl OnSend<N> for Send {
    fn send(&self, _mods: &mut Mods<N>, _send: SendMolecule<N>, _refs: Refs<N>) -> Vec<Name<N>> {
        self.names.clone()
    }
}
#[test]
fn polyadic() {
    // w[x,y].() w(a,b).()
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b) = (&n("a"), &n("b"));
    let actual = &Results::new();
    let wxy = read(w, Read::new(actual, &[x, y]));
    let wab = send(w, Send::new(&[a, b]));
    let mut pipl = Pipl::new();
    pipl.add(wxy);
    pipl.add(wab);
    pipl.step();
    let expected = &Results::new();
    expected.log("read", N::vec(vec![a.clone(), b.clone()]));
    assert_eq_results(actual, expected);
}
