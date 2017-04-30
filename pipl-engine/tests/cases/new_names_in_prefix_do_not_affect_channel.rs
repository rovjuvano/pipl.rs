use helpers::*;
#[test]
fn new_names_in_prefix_do_not_affect_channel() {
    // [w]w[m].(+ [x]x[n].(| [y]y[o].() ) )
    // [w]w(a).(+ [x]x(b).(| [y]y(c).() ) )
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b, c) = (&n("a"), &n("b"), &n("c"));
    let (m, n, o) = (&n("m"), &n("n"), &n("o"));
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    {
        let choice = builder
            .read(w).names(&[m]).restrict(&[w]).call(log("[w]w[m]", actual))
            .choice();
        let parallel = choice
            .read(x).names(&[n]).restrict(&[x]).call(log("[x]x[n]", actual))
            .parallel();
        parallel
            .read(y).names(&[o]).restrict(&[y]).call(log("[y]y[o]", actual));
    }
    {
        let choice = builder
            .send(w).names(&[a]).restrict(&[w]).call(log("[w]w(a)", actual))
            .choice();
        let parallel = choice
            .send(x).names(&[b]).restrict(&[x]).call(log("[x]x(b)", actual))
            .parallel();
        parallel
            .send(y).names(&[c]).restrict(&[y]).call(log("[y]y(c)", actual));
    }
    let mut pipl = Pipl::new();
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs_wm = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    refs_wm.set(w.clone(), w.dup());
    refs_wm.set(m.clone(), a.clone());
    refs_wa.set(w.clone(), w.dup());
    expected.log("[w]w[m]", refs_wm.clone());
    expected.log("[w]w(a)", refs_wa.clone());
    pipl.step();
    refs_wm.set(x.clone(), x.dup());
    refs_wm.set(n.clone(), b.clone());
    refs_wa.set(x.clone(), x.dup());
    expected.log("[x]x[n]", refs_wm.clone());
    expected.log("[x]x(b)", refs_wa.clone());
    pipl.step();
    refs_wm.set(y.clone(), y.dup());
    refs_wm.set(o.clone(), c.clone());
    refs_wa.set(y.clone(), y.dup());
    expected.log("[y]y[o]", refs_wm.clone());
    expected.log("[y]y(c)", refs_wa.clone());
    pipl.step();
    assert_eq_results(actual, expected);
    let refs_yo = actual.get(&"[y]y[o]").get(0).unwrap().clone();
    let refs_yc = actual.get(&"[y]y(c)").get(0).unwrap().clone();
    assert_ne_names(&refs_yo.get(w), w);
    assert_ne_names(&refs_yc.get(w), w);
    assert_ne_names(&refs_yo.get(x), x);
    assert_ne_names(&refs_yc.get(x), x);
    assert_ne_names(&refs_yo.get(y), y);
    assert_ne_names(&refs_yc.get(y), y);
}
