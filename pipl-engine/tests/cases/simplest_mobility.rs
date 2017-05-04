use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
    names: Vec<Name>,
    next: Option<(Name, Rc<Send>)>,
}
impl Read {
    fn new(results: &Rc<Results>, names: &[&Name]) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: names.iter().map(|&x| x.clone()).collect(),
            next: None,
        })
    }
    fn new_then(results: &Rc<Results>, names: &[&Name], channel: &Name, send: Rc<Send>) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: names.iter().map(|&x| x.clone()).collect(),
            next: Some((channel.clone(), send)),
        })
    }
}
impl OnRead for Read {
    fn read(&self, mods: &mut Mods, mut refs: Refs, names: Vec<Name>) {
        refs.set_names(self.names.clone(), names.clone());
        self.results.log("read", &refs);
        if let Some((ref channel, ref read)) = self.next {
            mods.send(channel, refs, read.clone());
        }
    }
}
#[derive(Debug)]
struct Send {
    names: Vec<Name>,
    next: Option<(Name, Rc<Read>)>,
}
impl Send {
    fn new(names: Vec<Name>) -> Rc<Self> {
        Rc::new(Send {
            names: names,
            next: None,
        })
    }
    fn new_then(names: Vec<Name>, channel: &Name, read: Rc<Read>) -> Rc<Self> {
        Rc::new(Send {
            names: names,
            next: Some((channel.clone(), read)),
        })
    }
}
impl OnSend for Send {
    fn send(&self, mods: &mut Mods, refs: Refs) -> Vec<Name> {
        let names = refs.get_names(&self.names);
        if let Some((ref channel, ref send)) = self.next {
            mods.read(channel, refs, send.clone());
        }
        names
    }
}
#[test]
fn simplest_mobility() {
    // w[z].z(z).() w(x).x[y].()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let actual = &Results::new();
    let read = Read::new_then(actual, &[z], z, Send::new(vec![z.clone()]));
    let send = Send::new_then(vec![x.clone()], x, Read::new(actual, &[y]));
    let mut pipl = Pipl::new();
    pipl.read(w, read);
    pipl.send(w, send);
    pipl.step();
    pipl.step();
    let expected = &Results::new();
    let refs_wz = &mut Refs::new();
    let refs_wx = &mut Refs::new();
    refs_wz.set(z.clone(), x.clone());
    expected.log("read", refs_wz);
    refs_wx.set(y.clone(), x.clone());
    expected.log("read", refs_wx);
    assert_eq_results(actual, expected);
}
