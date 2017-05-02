use helpers::*;
struct Read {
    results: Rc<Results>,
}
impl OnRead for Read {
    fn read(&mut self, _pipl: &mut Pipl, names: &Vec<Name>) {
        self.results.log("read", names);
    }
}
struct Send {
    names: Vec<Name>,
}
impl OnSend for Send {
    fn send(&mut self, _pipl: &mut Pipl) -> &Vec<Name> {
        &self.names
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
    pipl.read(w, read);
    pipl.send(w, send);
    pipl.step();
    let expected = Results::new();
    expected.log("read", &vec![a.clone()]);
    assert_eq!(actual, expected);
}
