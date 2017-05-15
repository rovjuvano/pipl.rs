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
impl OnRead for Read {
    fn read(&self, _mods: &mut Mods, _read: ReadMolecule, _refs: Refs, names: Vec<Name>) {
        self.results.log("read", Name::new(names));
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
impl OnSend for Send {
    fn send(&self, _mods: &mut Mods, _send: SendMolecule, _refs: Refs) -> Vec<Name> {
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
    expected.log("read", Name::new(vec![a.clone(), b.clone()]));
    assert_eq_results(actual, expected);
}
