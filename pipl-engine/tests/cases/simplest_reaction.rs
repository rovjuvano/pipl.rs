use helpers::*;
#[test]
fn simplest_reaction() {
    // w[x] w(a)
    let (w, x) = (&n("w"), &n("x"));
    let a = &n("a");
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![read(w, &[x])], Terminal, actual.clone()));
    pipl.add(make(vec![send(w, &[a])], Terminal, actual.clone()));
    pipl.step();
    let expected = Rc::new(Results::new());
    let refs = &mut Refs::new();
    expected.log(f(&send(w, &[a])), refs.clone());
    refs.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs.clone());
    assert_eq_results(actual, expected);
}
