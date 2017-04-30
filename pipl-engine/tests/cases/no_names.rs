use helpers::*;
#[test]
fn no_names() {
    // w[].x().() w(a).x[b].()
    let (w, x) = (&n("w"), &n("x"));
    let (a, b) = (&n("a"), &n("b"));
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    builder
        .read(w).call(log("w[]", actual))
        .send(x).call(log("x()", actual));
    builder
        .send(w).names(&[a]).call(log("w(a)", actual))
        .read(x).names(&[b]).call(log("x[b]", actual));
    let mut pipl = Pipl::new();
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs_empty = Refs::new();
    expected.log("w[]", refs_empty.clone());
    expected.log("x()", refs_empty.clone());
    expected.log("w(a)", refs_empty.clone());
    expected.log("x[b]", refs_empty.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
