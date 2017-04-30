use helpers::*;
#[test]
fn simplest_reaction() {
    // w[x] w(a)
    let (w, x) = (&n("w"), &n("x"));
    let a = &n("a");
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    builder.read(w).names(&[x]).call(log("w[x]", actual));
    builder.send(w).names(&[a]).call(log("w(a)", actual));
    let mut pipl = Pipl::new();
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs = &mut Refs::new();
    expected.log("w(a)", refs.clone());
    refs.set(x.clone(), a.clone());
    expected.log("w[x]", refs.clone());
    pipl.step();
    assert_eq_results(actual, expected);
}
