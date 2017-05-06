use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
    name: Name,
    names: Vec<Name>,
    next: Vec<Rc<Read>>,
}
impl Read {
    fn new(results: &Rc<Results>, name: &Name, names: &[&Name]) -> Rc<Self> {
        Self::new_then(results, name, names, Vec::new())
    }
    fn new_then(results: &Rc<Results>, name: &Name, names: &[&Name], next: Vec<Rc<Read>>) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            name: name.clone(),
            names: names.iter().map(|&x| x.clone()).collect(),
            next: next,
        })
    }
}
impl OnRead for Read {
    fn read(&self, mods: &mut Mods, mut refs: Refs, names: Vec<Name>) {
        refs.set_names(self.names.clone(), names.clone());
        self.results.log(format!("{}", self.name.raw().downcast_ref::<&str>().unwrap()), Name::new(refs.clone()));
        for read in self.next.iter() {
            mods.read(&refs.get(&read.name), refs.clone(), read.clone());
        }
    }
}
#[derive(Debug)]
struct Send {
    names: Vec<Name>,
    next: Option<(Name, Rc<Send>)>,
}
impl Send {
    fn new(names: &[&Name]) -> Rc<Self> {
        Rc::new(Send {
            names: names.iter().map(|&x| x.clone()).collect(),
            next: None,
        })
    }
    fn new_then(names: &[&Name], channel: &Name, send: Rc<Send>) -> Rc<Self> {
        Rc::new(Send {
            names: names.iter().map(|&x| x.clone()).collect(),
            next: Some((channel.clone(), send)),
        })
    }
}
impl OnSend for Send {
    fn send(&self, mods: &mut Mods, refs: Refs) -> Vec<Name> {
        let names = refs.get_names(&self.names);
        if let Some((ref channel, ref send)) = self.next {
            mods.send(channel, refs, send.clone());
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
    let wx = Read::new_then(actual, w, &[x], vec![
        Read::new(actual, x, &[y]),
        Read::new_then(actual, y, &[z], vec![
            Read::new(actual, y, &[z])
        ]),
    ]);
    let wa = Send::new_then(&[a],
        a, Send::new_then(&[b],
            y, Send::new(&[c])
        )
    );
    let bd = Send::new(&[d]);
    let mut pipl = Pipl::new();
    pipl.read(w, wx);
    pipl.send(w, wa);
    pipl.send(b, bd);
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
