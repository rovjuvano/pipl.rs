use helpers::*;
#[test]
fn multi_step_reaction() {
    // w[x].w[y] w(a).w(b)
    let mut pipl = Pipl::new();
    names!(|pipl| { w x y a b });
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    builder
        .read(w).names(&[x]).call(log("w[x]", actual))
        .read(w).names(&[y]).call(log("w[y]", actual));
    builder
        .send(w).names(&[a]).call(log("w(a)", actual))
        .send(w).names(&[b]).call(log("w(b)", actual));
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs = &mut Refs::new();
    expected.log("w(a)", refs.clone());
    expected.log("w(b)", refs.clone());
    refs.insert(x.clone(), a.clone());
    expected.log("w[x]", refs.clone());
    refs.insert(y.clone(), b.clone());
    expected.log("w[y]", refs.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
}
