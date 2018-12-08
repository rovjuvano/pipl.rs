use helpers::*;
#[test]
fn new_names_in_parallel_prefixes() {
    // w[x].(| [x]w(x).x(b).() [a]w[y].a(c).() x[z].() ) w(a).a[z].()
    let mut pipl = Pipl::new();
    names!(|pipl| { w x y z a b c });
    let actual = &Results::new();
    let mut builder = PiplBuilder::new();
    {
        let parallel = builder
            .read(w).names(&[x]).call(log("w[x]", actual))
            .parallel();
        parallel
            .send(w).names(&[x]).restrict(&[x]).call(log("[x]w(x)", actual))
            .send(x).names(&[b]).call(log("x(b)", actual));
        parallel
            .read(w).names(&[y]).restrict(&[a]).call(log("[a]w[y]", actual))
            .send(a).names(&[c]).call(log("a(c)", actual));
        parallel
            .read(x).names(&[z]).call(log("x[z]", actual));
    }
    builder
        .send(w).names(&[a]).call(log("w(a)", actual))
        .read(a).names(&[z]).call(log("a[z]", actual));
    builder.apply(&mut pipl);
    let expected = &Results::new();
    let refs_wx = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    refs_wx.insert(x.clone(), a.clone());
    expected.log("w[x]", refs_wx.clone());
    expected.log("w(a)", refs_wa.clone());
    pipl.step();
    let refs_wxwx = &mut refs_wx.clone();
    let refs_wxwy = &mut refs_wx.clone();
    let x2 = pipl.dup_name(x);
    refs_wxwx.insert(x.clone(), x2.clone());
    refs_wxwy.insert(y.clone(), x2.clone());
    refs_wxwy.insert(a.clone(), pipl.dup_name(a));
    expected.log("[a]w[y]", refs_wxwy.clone());
    expected.log("[x]w(x)", refs_wxwx.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
    assert_ne_names(&actual.get(&"[a]w[y]").get(0).unwrap().get(a).unwrap(), a);
    assert_ne_names(&actual.get(&"[x]w(x)").get(0).unwrap().get(x).unwrap(), x);
    assert_eq!(
        &actual.get(&"[a]w[y]").get(0).unwrap().get(y),
        &actual.get(&"[x]w(x)").get(0).unwrap().get(x)
    );
}