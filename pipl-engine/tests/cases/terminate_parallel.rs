use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
    names: Vec<Name>,
    next: Vec<Molecule>,
}
impl Read {
    fn new(results: &Rc<Results>, names: &[&Name]) -> Rc<Self> {
        Self::new_then(results, names, Vec::new())
    }
    fn new_then(results: &Rc<Results>, names: &[&Name], next: Vec<Molecule>) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: unslice(names),
            next: next,
        })
    }
}
impl OnRead for Read {
    fn read(&self, mods: &mut Mods, read: ReadMolecule, mut refs: Refs, names: Vec<Name>) {
        refs.set_names(self.names.clone(), names.clone());
        self.results.log(format!("{}", read.name().raw().downcast_ref::<&str>().unwrap()), Name::new(refs.clone()));
        for next in self.next.iter() {
            mods.add(next.clone(), refs.clone());
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
        let names = refs.get_names(&self.names);
        if let Some(ref next) = self.next {
            mods.add(next.clone(), refs);
        }
        names
    }
}
#[test]
fn terminate_parallel() {
    // w[x].(| x[y].() y[z].y[z].() ) w(a).a(b).y(c).() b(d).()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let (a, b, c, d) = (&n("a"), &n("b"), &n("c"), &n("d"));
    let actual = &Results::new();
    let wx = read(w, Read::new_then(actual, &[x], vec![
        read(x, Read::new(actual, &[y])),
        read(y, Read::new_then(actual, &[z], vec![
            read(y, Read::new(actual, &[z]))
        ]))
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
    expected.log("w", Name::new(refs_wx.clone()));
    let refs_wxxy = &mut refs_wx.clone();
    refs_wxxy.set(y.clone(), b.clone());
    expected.log("x", Name::new(refs_wxxy.clone()));
    let refs_wxyz = &mut refs_wx.clone();
    refs_wxyz.set(z.clone(), c.clone());
    expected.log("y", Name::new(refs_wxyz.clone()));
    pipl.step();
    pipl.step();
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
