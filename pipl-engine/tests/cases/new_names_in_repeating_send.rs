use helpers::*;
#[test]
fn new_names_in_repeating_send() {
    // w[x].w[y].y(b).x(c).() ![a]w(a).a[x].() !a(d).()
    let mut pipl = Pipl::new();
    names!(|pipl| { w x y a b c d });
    let actual = &Results::new();
    let mut builder = PiplBuilder::new();
    builder
        .read(w).names(&[x]).call(log("w[x]", actual))
        .read(w).names(&[y]).call(log("w[y]", actual))
        .send(y).names(&[b]).call(log("y(b)", actual))
        .send(x).names(&[c]).call(log("x(c)", actual));
    builder
        .send(w).names(&[a]).call(log("![a]w(a)", actual)).repeat().restrict(&[a])
        .read(a).names(&[x]).call(log("a[x]", actual));
    builder
        .send(a).names(&[d]).repeat().call(log("!a(d)", actual));
    builder.apply(&mut pipl);
    let expected = &Results::new();
    let refs_wx = &mut Refs::new();
    let refs_wax = &mut Refs::new();
    let refs_way = &mut Refs::new();
    refs_wx.insert(x.clone(), a.clone());
    refs_wax.insert(a.clone(), a.clone());
    expected.log("w[x]", refs_wx.clone());
    expected.log("![a]w(a)", refs_wax.clone());
    pipl.step();
    refs_wx.insert(y.clone(), a.clone());
    refs_way.insert(a.clone(), a.clone());
    expected.log("w[y]", refs_wx.clone());
    expected.log("![a]w(a)", refs_way.clone());
    pipl.step();
    refs_way.insert(x.clone(), b.clone());
    expected.log("y(b)", refs_wx.clone());
    expected.log("a[x]", refs_way.clone());
    pipl.step();
    refs_wax.insert(x.clone(), c.clone());
    expected.log("x(c)", refs_wx.clone());
    expected.log("a[x]", refs_wax.clone());
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
}
