use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
    names: Vec<Name>,
    next: Option<Molecule<N>>,
}
impl Read {
    fn new(results: &Rc<Results>, names: &[&Name]) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: unslice(names),
            next: None,
        })
    }
    fn new_then(results: &Rc<Results>, names: &[&Name], next: Molecule<N>) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: unslice(names),
            next: Some(next),
        })
    }
}
impl OnRead<N> for Read {
    fn read(&self, mods: &mut Mods<N>, _read: ReadMolecule<N>, refs: Refs, names: Vec<Name>) {
        self.results.log("read", N::Vec(names));
        if let Some(ref next) = self.next {
            mods.add(next.clone(), refs);
        }
    }
}
#[derive(Debug)]
struct Send {
    names: Vec<Name>,
    next: Option<Molecule<N>>,
}
impl Send {
    fn new(names: &[&Name]) -> Rc<Self> {
        Rc::new(Send {
            names: unslice(names),
            next: None,
        })
    }
    fn new_then(names: &[&Name], next: Molecule<N>) -> Rc<Self> {
        Rc::new(Send {
            names: unslice(names),
            next: Some(next),
        })
    }
}
impl OnSend<N> for Send {
    fn send(&self, mods: &mut Mods<N>, _send: SendMolecule<N>, refs: Refs) -> Vec<Name> {
        if let Some(ref next) = self.next {
            mods.add(next.clone(), refs);
        }
        self.names.clone()
    }
}
#[test]
fn no_names() {
    // w[].x().() w(a).x[b].()
    let mut pipl = Pipl::new();
    names!(|pipl| { w x a b });
    let actual = &Results::new();
    let w_ = read(w, Read::new_then(actual, &[], send(x, Send::new(&[]))));
    let wa = send(w, Send::new_then(&[a], read(x, Read::new(actual, &[b]))));
    pipl.add(w_);
    pipl.add(wa);
    pipl.step();
    pipl.step();
    let expected = &Results::new();
    expected.log("read", N::Vec(vec![a.clone()]));
    expected.log("read", N::Vec(vec![]));
    assert_eq_results(actual, expected);
}
