use helpers::*;
#[test]
fn new_names_in_choice_prefixes() {
    // w[x].(+ [m]w[y].y(m).m(b).() )
    // w(a).(+ [n]w(n).n[o].o[p].() )
    // m[z].() n(c).() o(d).()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let (a, b, c, d) = (&n("a"), &n("b"), &n("c"), &n("d"));
    let (m, n, o, p) = (&n("m"), &n("n"), &n("o"), &n("p"));
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    {
        let choice = builder
            .read(w).names(&[x]).call(log("w[x]", actual))
            .choice();
        choice
            .read(w).names(&[y]).restrict(&[m]).call(log("[m]w[y]", actual))
            .send(y).names(&[m]).call(log("y(m)", actual))
            .send(m).names(&[b]).call(log("m(b)", actual));
    }
    {
        let choice = builder
            .send(w).names(&[a]).call(log("w(a)", actual))
            .choice();
        choice
            .send(w).names(&[n]).restrict(&[n]).call(log("[n]w(n)", actual))
            .read(n).names(&[o]).call(log("n[o]", actual))
            .read(o).names(&[p]).call(log("o[p]", actual));
    }
    builder
        .read(m).names(&[z]).call(log("m[z]", actual));
    builder
        .send(n).names(&[c]).call(log("n(c)", actual));
    builder
        .send(o).names(&[d]).call(log("o(d)", actual));
    let mut pipl = Pipl::new();
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log("w[x]", refs_wx.clone());
    expected.log("w(a)", refs_wa.clone());
    pipl.step();
    let m2 = m.dup();
    let n2 = n.dup();
    refs_wx.set(m.clone(), m2.clone());
    refs_wx.set(y.clone(), n2.clone());
    refs_wa.set(n.clone(), n2.clone());
    expected.log("[m]w[y]", refs_wx.clone());
    expected.log("[n]w(n)", refs_wa.clone());
    pipl.step();
    refs_wa.set(o.clone(), m2.clone());
    expected.log("n[o]", refs_wa.clone());
    expected.log("y(m)", refs_wx.clone());
    pipl.step();
    refs_wa.set(p.clone(), b.clone());
    expected.log("o[p]", refs_wa.clone());
    expected.log("m(b)", refs_wx.clone());
    pipl.step();
    assert_eq_results(actual, expected);
}
