use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
    names: Vec<Name>,
    next: Option<Molecule>,
}
impl Read {
    fn new(results: &Rc<Results>, names: &[&Name]) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: unslice(names),
            next: None,
        })
    }
    fn new_then(results: &Rc<Results>, names: &[&Name], next: Molecule) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: unslice(names),
            next: Some(next),
        })
    }
}
impl OnRead for Read {
    fn read(&self, mods: &mut Mods, _read: ReadMolecule, refs: Refs, names: Vec<Name>) {
        self.results.log("read", Name::new(names));
        if let Some(ref next) = self.next {
            mods.add(next.clone(), refs);
        }
    }
}
#[derive(Debug)]
struct Send {
    names: Vec<Name>,
    next: Option<Molecule>,
}
impl Send {
    fn new(names: &[&Name]) -> Rc<Self> {
        Rc::new(Send {
            names: unslice(names),
            next: None,
        })
    }
    fn new_then(names: &[&Name], next: Molecule) -> Rc<Self> {
        Rc::new(Send {
            names: unslice(names),
            next: Some(next),
        })
    }
}
impl OnSend for Send {
    fn send(&self, mods: &mut Mods, _send: SendMolecule, refs: Refs) -> Vec<Name> {
        if let Some(ref next) = self.next {
            mods.add(next.clone(), refs);
        }
        self.names.clone()
    }
}
#[test]
fn no_names() {
    // w[].x().() w(a).x[b].()
    let (w, x) = (&n("w"), &n("x"));
    let (a, b) = (&n("a"), &n("b"));
    let actual = &Results::new();
    let w_ = read(w, Read::new_then(actual, &[], send(x, Send::new(&[]))));
    let wa = send(w, Send::new_then(&[a], read(x, Read::new(actual, &[b]))));
    let mut pipl = Pipl::new();
    pipl.add(w_);
    pipl.add(wa);
    pipl.step();
    pipl.step();
    let expected = &Results::new();
    expected.log("read", Name::new(vec![a.clone()]));
    expected.log("read", Name::new(vec![] as Vec<Name>));
    assert_eq_results(actual, expected);
}
