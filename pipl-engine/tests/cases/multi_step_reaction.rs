use helpers::*;
#[test]
fn multi_step_reaction() {
    // w[x].w[y] w(a).w(b)
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b) = (&n("a"), &n("b"));
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    builder
        .read(w).names(&[x]).call(log("w[x]", actual))
        .read(w).names(&[y]).call(log("w[y]", actual));
    builder
        .send(w).names(&[a]).call(log("w(a)", actual))
        .send(w).names(&[b]).call(log("w(b)", actual));
    let mut pipl = Pipl::new();
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs = &mut Refs::new();
    expected.log("w(a)", refs.clone());
    expected.log("w(b)", refs.clone());
    refs.set(x.clone(), a.clone());
    expected.log("w[x]", refs.clone());
    refs.set(y.clone(), b.clone());
    expected.log("w[y]", refs.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
