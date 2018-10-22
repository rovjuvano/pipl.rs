use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
    names: Vec<Name<N>>,
    next: Option<Molecule<N>>,
}
impl Read {
    fn new(results: &Rc<Results>, names: &[&Name<N>]) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: unslice(names),
            next: None,
        })
    }
    fn new_then(results: &Rc<Results>, names: &[&Name<N>], next: Molecule<N>) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: unslice(names),
            next: Some(next),
        })
    }
}
impl OnRead<N> for Read {
    fn read(&self, mods: &mut Mods<N>, _read: ReadMolecule<N>, refs: Refs<N>, names: Vec<Name<N>>) {
        self.results.log("read", N::vec(names));
        if let Some(ref next) = self.next {
            mods.add(next.clone(), refs);
        }
    }
}
#[derive(Debug)]
struct Send {
    names: Vec<Name<N>>,
    next: Option<Molecule<N>>,
}
impl Send {
    fn new(names: &[&Name<N>]) -> Rc<Self> {
        Rc::new(Send {
            names: unslice(names),
            next: None,
        })
    }
    fn new_then(names: &[&Name<N>], next: Molecule<N>) -> Rc<Self> {
        Rc::new(Send {
            names: unslice(names),
            next: Some(next),
        })
    }
}
impl OnSend<N> for Send {
    fn send(&self, mods: &mut Mods<N>, _send: SendMolecule<N>, refs: Refs<N>) -> Vec<Name<N>> {
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
    expected.log("read", N::vec(vec![a.clone()]));
    expected.log("read", N::vec(vec![]));
    assert_eq_results(actual, expected);
}
