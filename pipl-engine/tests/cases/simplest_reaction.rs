use helpers::*;
#[test]
fn simplest_reaction() {
    // w[x] w(a)
    let (w, a) = (&n("w"), &n("a"));
    let actual = &Rc::new(Results::new());
    let mut pipl = Pipl::new();
    {
        pipl.read(w, |names| {
            actual.log("x", &names[0]);
        });
    }
    {
        pipl.send(w, || {
            vec![a.clone()]
        });
    }
    let expected = &Rc::new(Results::new());
    expected.log("x", a);
    pipl.step();
    assert_eq_results(actual, expected);
}
