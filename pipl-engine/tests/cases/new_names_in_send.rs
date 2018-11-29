use helpers::*;
#[test]
fn new_names_in_send() {
    // w[x].[w, x]y(w).a(b).x(c).() w(a).y[m].a[n].x[o].()
    let mut pipl = Pipl::new();
    names!(|pipl| { w x y a b c m n o });
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    builder
        .read(w).names(&[x]).call(log("w[x]", actual))
        .send(y).names(&[w]).call(log("y(w)", actual)).restrict(&[w, x])
        .send(a).names(&[b]).call(log("a(b)", actual))
        .send(x).names(&[c]).call(log("x(c)", actual));
    builder
        .send(w).names(&[a]).call(log("w(a)", actual))
        .read(y).names(&[m]).call(log("y[m]", actual))
        .read(a).names(&[n]).call(log("a[n]", actual))
        .read(x).names(&[o]).call(log("x[o]", actual));
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    refs_wx.insert(x.clone(), a.clone());
    expected.log("w[x]", refs_wx.clone());
    expected.log("w(a)", refs_wa.clone());
    pipl.step();
    let w2 = &pipl.dup_name(w);
    refs_wx.insert(w.clone(), w2.clone());
    refs_wx.insert(x.clone(), pipl.dup_name(x));
    refs_wa.insert(m.clone(), w2.clone());
    expected.log("y[m]", refs_wa.clone());
    expected.log("y(w)", refs_wx.clone());
    pipl.step();
    refs_wa.insert(n.clone(), b.clone());
    expected.log("a[n]", refs_wa.clone());
    expected.log("a(b)", refs_wx.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
    assert_ne_names(&actual.get(&"y(w)").get(0).unwrap().get(w).unwrap(), w);
    assert_ne_names(&actual.get(&"y[m]").get(0).unwrap().get(m).unwrap(), w);
}
