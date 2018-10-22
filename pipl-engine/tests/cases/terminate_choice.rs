use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
    names: Vec<Name<N>>,
    next: Vec<Molecule<N>>,
}
impl Read {
    fn new(results: &Rc<Results>, names: &[&Name<N>]) -> Rc<Self> {
        Self::new_then(results, names, Vec::new())
    }
    fn new_then(results: &Rc<Results>, names: &[&Name<N>], next: Vec<Molecule<N>>) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: unslice(names),
            next: next,
        })
    }
}
impl OnRead<N> for Read {
    fn read(&self, mods: &mut Mods<N>, read: ReadMolecule<N>, mut refs: Refs<N>, names: Vec<Name<N>>) {
        refs.set_names(self.names.clone(), names.clone());
        self.results.log(read.name().raw(), N::refs(refs.clone()));
        mods.choice(self.next.clone(), refs);
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
        let names = refs.get_names(&self.names);
        if let Some(ref next) = self.next {
            mods.add(next.clone(), refs);
        }
        names
    }
}
#[test]
fn terminate_choice() {
    // w[x].(+ x[y].() y[z].y[z].() ) w(a).a(b).y(c).() b(d).()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let (a, b, c, d) = (&n("a"), &n("b"), &n("c"), &n("d"));
    let actual = &Results::new();
    let wx = read(w, Read::new_then(actual, &[x], vec![
        read(x, Read::new(actual, &[y])),
        read(y, Read::new_then(actual, &[z], vec![
            read(y, Read::new(actual, &[z]))
        ])),
    ]));
    let wa = send(w, Send::new_then(&[a],
        send(a, Send::new_then(&[b],
            send(y, Send::new(&[c]))
        ))
    ));
    let bd = send(b, Send::new(&[d]));
    let mut pipl = Pipl::new();
    pipl.add(wx);
    pipl.add(wa);
    pipl.add(bd);
    let expected = &Results::new();
    let refs_wx = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log("w", N::refs(refs_wx.clone()));
    refs_wx.set(y.clone(), b.clone());
    expected.log("x", N::refs(refs_wx.clone()));
    pipl.step();
    pipl.step();
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
