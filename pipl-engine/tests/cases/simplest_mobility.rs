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
    fn read(&self, mods: &mut Mods, _read: ReadMolecule, mut refs: Refs, names: Vec<Name>) {
        refs.set_names(self.names.clone(), names.clone());
        self.results.log("read", Name::new(refs.clone()));
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
        let names = refs.get_names(&self.names);
        if let Some(ref next) = self.next {
            mods.add(next.clone(), refs);
        }
        names
    }
}
#[test]
fn simplest_mobility() {
    // w[z].z(z).() w(x).x[y].()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let actual = &Results::new();
    let wz = read(w, Read::new_then(actual, &[z], send(z, Send::new(&[z]))));
    let wx = send(w, Send::new_then(&[x], read(x, Read::new(actual, &[y]))));
    let mut pipl = Pipl::new();
    pipl.add(wz);
    pipl.add(wx);
    pipl.step();
    pipl.step();
    let expected = &Results::new();
    let refs_wz = &mut Refs::new();
    let refs_wx = &mut Refs::new();
    refs_wz.set(z.clone(), x.clone());
    expected.log("read", Name::new(refs_wz.clone()));
    refs_wx.set(y.clone(), x.clone());
    expected.log("read", Name::new(refs_wx.clone()));
    assert_eq_results(actual, expected);
}
