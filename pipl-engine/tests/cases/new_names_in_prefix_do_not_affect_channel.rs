use helpers::*;
#[test]
fn new_names_in_prefix_do_not_affect_channel() {
    // [w]w[m].(+ [x]x[n].(| [y]y[o].() ) )
    // [w]w(a).(+ [x]x(b).(| [y]y(c).() ) )
    let mut pipl = Pipl::new();
    names!(|pipl| { w x y a b c m n o });
    let actual = &Results::new();
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
    builder.apply(&mut pipl);
    let expected = &Results::new();
    let refs_wm = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    refs_wm.insert(w.clone(), w.clone());
    refs_wm.insert(m.clone(), a.clone());
    refs_wa.insert(w.clone(), w.clone());
    expected.log("[w]w[m]", refs_wm.clone());
    expected.log("[w]w(a)", refs_wa.clone());
    pipl.step();
    refs_wm.insert(x.clone(), x.clone());
    refs_wm.insert(n.clone(), b.clone());
    refs_wa.insert(x.clone(), x.clone());
    expected.log("[x]x[n]", refs_wm.clone());
    expected.log("[x]x(b)", refs_wa.clone());
    pipl.step();
    refs_wm.insert(y.clone(), y.clone());
    refs_wm.insert(o.clone(), c.clone());
    refs_wa.insert(y.clone(), y.clone());
    expected.log("[y]y[o]", refs_wm.clone());
    expected.log("[y]y(c)", refs_wa.clone());
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
    let refs_yo = actual.get(&"[y]y[o]").get(0).unwrap().clone();
    let refs_yc = actual.get(&"[y]y(c)").get(0).unwrap().clone();
    assert_ne_names(&refs_yo.get(w).unwrap(), w);
    assert_ne_names(&refs_yc.get(w).unwrap(), w);
    assert_ne_names(&refs_yo.get(x).unwrap(), x);
    assert_ne_names(&refs_yc.get(x).unwrap(), x);
    assert_ne_names(&refs_yo.get(y).unwrap(), y);
    assert_ne_names(&refs_yc.get(y).unwrap(), y);
}
