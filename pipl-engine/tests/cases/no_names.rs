use helpers::*;
#[test]
fn no_names() {
    // w[].x().() w(a).x[b].()
    let (w, x) = (&n("w"), &n("x"));
    let (a, b) = (&n("a"), &n("b"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![read(w, &[]), send(x, &[])], Terminal, actual.clone()));
    pipl.add(make(vec![send(w, &[a]), read(x, &[b])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = Refs::new();
    expected.log(f(&read(w, &[])), refs_empty.clone());
    expected.log(f(&send(x, &[])), refs_empty.clone());
    expected.log(f(&send(w, &[a])), refs_empty.clone());
    expected.log(f(&read(x, &[b])), refs_empty.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
