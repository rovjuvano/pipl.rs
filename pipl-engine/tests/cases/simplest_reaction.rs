use helpers::*;
#[test]
fn simplest_reaction() {
    // w[x] w(a)
    let mut pipl = Pipl::new();
    names!(|pipl| { w x a });
    let actual = &Results::new();
    let mut builder = PiplBuilder::new();
    builder.read(w).names(&[x]).call(log("w[x]", actual));
    builder.send(w).names(&[a]).call(log("w(a)", actual));
    builder.apply(&mut pipl);
    let expected = &Results::new();
    let refs = &mut Refs::new();
    expected.log("w(a)", refs.clone());
    refs.insert(x.clone(), a.clone());
    expected.log("w[x]", refs.clone());
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
}
