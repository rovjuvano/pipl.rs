use helpers::*;
#[test]
fn no_names() {
    // w[].x().() w(a).x[b].()
    let mut pipl = Pipl::new();
    names!(|pipl| { w x a b });
    let actual = &Results::new();
    let mut builder = PiplBuilder::new();
    builder
        .read(w).call(log("w[]", actual))
        .send(x).call(log("x()", actual));
    builder
        .send(w).names(&[a]).call(log("w(a)", actual))
        .read(x).names(&[b]).call(log("x[b]", actual));
    builder.apply(&mut pipl);
    let expected = &Results::new();
    let refs_empty = Refs::new();
    expected.log("w[]", refs_empty.clone());
    expected.log("x()", refs_empty.clone());
    expected.log("w(a)", refs_empty.clone());
    expected.log("x[b]", refs_empty.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
}
