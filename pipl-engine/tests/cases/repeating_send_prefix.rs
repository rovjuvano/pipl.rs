use helpers::*;
#[test]
fn repeating_send_prefix() {
    // w[a].a[c].w[b].b[c].a[d].b[e].() !w(x).!x(y).()
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b, c, d, e) = (&n("a"), &n("b"), &n("c"), &n("d"), &n("e"));
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    builder
        .read(w).names(&[a]).call(log("w[a]", actual))
        .read(a).names(&[c]).call(log("a[c]", actual))
        .read(w).names(&[b]).call(log("w[b]", actual))
        .read(b).names(&[c]).call(log("b[c]", actual))
        .read(a).names(&[d]).call(log("a[d]", actual))
        .read(b).names(&[e]).call(log("b[e]", actual));
    builder
        .send(w).names(&[x]).repeat().call(log("!w(x)", actual))
        .send(x).names(&[y]).repeat().call(log("!x(y)", actual));
    let mut pipl = Pipl::new();
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs_empty = Refs::new();
    let refs_wa = &mut Refs::new();
    // w[a].a[c] !w(x).!x(y)
    refs_wa.set(a.clone(), x.clone());
    expected.log("w[a]", refs_wa.clone());
    refs_wa.set(c.clone(), y.clone());
    expected.log("a[c]", refs_wa.clone());
    expected.log("!w(x)", refs_empty.clone());
    expected.log("!x(y)", refs_empty.clone());
    pipl.step();
    pipl.step();
    // w[b].b[c] !w(x).!x(y)
    refs_wa.set(b.clone(), x.clone());
    expected.log("w[b]", refs_wa.clone());
    refs_wa.set(c.clone(), y.clone());
    expected.log("b[c]", refs_wa.clone());
    expected.log("!w(x)", refs_empty.clone());
    expected.log("!x(y)", refs_empty.clone());
    pipl.step();
    pipl.step();
    // a[d].b[e] !x(y).()
    refs_wa.set(d.clone(), y.clone());
    expected.log("a[d]", refs_wa.clone());
    refs_wa.set(e.clone(), y.clone());
    expected.log("b[e]", refs_wa.clone());
    expected.log("!x(y)", refs_empty.clone());
    expected.log("!x(y)", refs_empty.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
