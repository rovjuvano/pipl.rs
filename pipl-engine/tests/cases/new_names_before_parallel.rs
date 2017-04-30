use helpers::*;
#[test]
fn new_names_before_parallel() {
    // w[x].[w, x](| w(c).() x[o].() y[p].x(p).x(p) )
    // w(a).w[m].() a[n].() x[o].() y(b).()
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b, c) = (&n("a"), &n("b"), &n("c"));
    let (m, n, o, p) = (&n("m"), &n("n"), &n("o"), &n("p"));
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    {
        let parallel = builder
            .read(w).names(&[x]).call(log("w[x]", actual))
            .parallel();
        parallel.restrict(&[w, x]);
        parallel
            .send(w).names(&[c]).call(log("w(c)", actual));
        parallel
            .read(x).names(&[o]).call(log("x[o]", actual));
        parallel
            .read(y).names(&[p]).call(log("y[p]", actual))
            .send(x).names(&[p]).call(log("x(p)", actual))
            .send(x).names(&[p]).call(log("x(p)", actual));
    }
    builder
        .send(w).names(&[a]).call(log("w(a)", actual))
        .read(w).names(&[m]).call(log("w[m]", actual));
    builder
        .read(a).names(&[n]).call(log("a[n]", actual));
    builder
        .read(x).names(&[o]).call(log("x[o]", actual));
    builder
        .send(y).names(&[b]).call(log("y(b)", actual));
    let mut pipl = Pipl::new();
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    let refs_yb = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log("w[x]", refs_wx.clone());
    expected.log("w(a)", refs_wa.clone());
    pipl.step();
    refs_wx.set(w.clone(), w.dup());
    refs_wx.set(x.clone(), x.dup());
    let refs_wxxo = &mut refs_wx.clone();
    let refs_wxyp = &mut refs_wx.clone();
    refs_wxyp.set(p.clone(), b.clone());
    expected.log("y[p]", refs_wxyp.clone());
    expected.log("y(b)", refs_yb.clone());
    pipl.step();
    refs_wxxo.set(o.clone(), b.clone());
    expected.log("x[o]", refs_wxxo.clone());
    expected.log("x(p)", refs_wxyp.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
