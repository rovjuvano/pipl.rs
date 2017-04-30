use helpers::*;
#[test]
fn terminate_choice() {
    // w[x].(+ x[y].() y[z].y[z].() ) w(a).a(b).y(c).() b(d).()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let (a, b, c, d) = (&n("a"), &n("b"), &n("c"), &n("d"));
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    {
        let choice = builder.read(w).names(&[x]).call(log("w[x]", actual)).choice();
        choice
            .read(x).names(&[y]).call(log("x[y]", actual));
        choice
            .read(y).names(&[z]).call(log("y[z]", actual))
            .read(y).names(&[z]).call(log("y[z]", actual));
    }
    builder
        .send(w).names(&[a]).call(log("w(a)", actual))
        .send(a).names(&[b]).call(log("a(b)", actual))
        .send(y).names(&[c]).call(log("y(c)", actual));
    builder.send(b).names(&[d]).call(log("b(d)", actual));
    let mut pipl = Pipl::new();
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs_empty = Refs::new();
    let refs_wx = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log("w[x]", refs_wx.clone());
    let refs_wxxy = &mut refs_wx.clone();
    refs_wxxy.set(y.clone(), b.clone());
    expected.log("x[y]", refs_wxxy.clone());
    expected.log("w(a)", refs_empty.clone());
    expected.log("a(b)", refs_empty.clone());
    pipl.step();
    pipl.step();
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}