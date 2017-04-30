use helpers::*;
#[test]
fn simplest_mobility() {
    // w(x).x[y].() w[z].z(z).()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    builder
        .send(w).names(&[x]).call(log("w(x)", actual))
        .read(x).names(&[y]).call(log("x[y]", actual));
    builder
        .read(w).names(&[z]).call(log("w[z]", actual))
        .send(z).names(&[z]).call(log("z(z)", actual));
    let mut pipl = Pipl::new();
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wz = &mut Refs::new();
    refs_wz.set(z.clone(), x.clone());
    expected.log("w(x)", refs_wx.clone());
    expected.log("w[z]", refs_wz.clone());
    pipl.step();
    assert_eq_results(actual, expected);
    refs_wx.set(y.clone(), x.clone());
    expected.log("x[y]", refs_wx.clone());
    expected.log("z(z)", refs_wz.clone());
    pipl.step();
    assert_eq_results(actual, expected);
}
