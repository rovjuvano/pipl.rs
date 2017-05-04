use helpers::*;
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
}
impl OnRead for Read {
    fn read(&self, _mods: &mut Mods, _refs: Refs, names: Vec<Name>) {
        self.results.log("read", &names);
    }
}
#[derive(Debug)]
struct Send {
    names: Vec<Name>,
}
impl OnSend for Send {
    fn send(&self, _mods: &mut Mods, _refs: Refs) -> Vec<Name> {
        self.names.clone()
    }
}
#[test]
fn simplest_reaction() {
    // w[x] w(a)
    let (w, a) = (&n("w"), &n("a"));
    let actual = Results::new();
    let read = Read { results: actual.clone() };
    let send = Send { names: vec![a.clone()] };
    let mut pipl = Pipl::new();
    pipl.read(w, Rc::new(read));
    pipl.send(w, Rc::new(send));
    pipl.step();
    let expected = Results::new();
    expected.log("read", &vec![a.clone()]);
    assert_eq!(actual, expected);
}
