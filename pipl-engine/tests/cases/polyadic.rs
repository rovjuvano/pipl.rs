use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
    names: Vec<Name>,
}
impl Read {
    fn new(results: &Rc<Results>, names: &[&Name]) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: unslice(names),
        })
    }
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
impl Send {
    fn new(names: &[&Name]) -> Rc<Self> {
        Rc::new(Send {
            names: unslice(names),
        })
    }
}
impl OnSend<N> for Send {
    fn send(&self, _mods: &mut Mods<N>, _send: SendMolecule<N>, _refs: Refs) -> Vec<Name> {
        self.names.clone()
    }
}
#[test]
fn polyadic() {
    // w[x,y].() w(a,b).()
    let mut pipl = Pipl::new();
    names!(|pipl| { w x y a b });
    let actual = &Results::new();
    let wxy = read(w, Read::new(actual, &[x, y]));
    let wab = send(w, Send::new(&[a, b]));
    pipl.add(wxy);
    pipl.add(wab);
    pipl.step();
    let expected = &Results::new();
    expected.log("read", N::Vec(vec![a.clone(), b.clone()]));
    assert_eq_results(actual, expected);
}
