use helpers::*;
#[test]
fn simplest_mobility() {
    // w(x).x[y].() w[z].z(z).()
    let mut pipl = Pipl::new();
    names!(|pipl| { w x y z });
    let actual = &Results::new();
    let mut builder = PiplBuilder::new();
    builder
        .send(w).names(&[x]).call(log("w(x)", actual))
        .read(x).names(&[y]).call(log("x[y]", actual));
    builder
        .read(w).names(&[z]).call(log("w[z]", actual))
        .send(z).names(&[z]).call(log("z(z)", actual));
    builder.apply(&mut pipl);
    let expected = &Results::new();
    let refs_wx = &mut Refs::new();
    let refs_wz = &mut Refs::new();
    refs_wz.insert(z.clone(), x.clone());
    expected.log("w(x)", refs_wx.clone());
    expected.log("w[z]", refs_wz.clone());
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
    refs_wx.insert(y.clone(), x.clone());
    expected.log("x[y]", refs_wx.clone());
    expected.log("z(z)", refs_wz.clone());
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
}
