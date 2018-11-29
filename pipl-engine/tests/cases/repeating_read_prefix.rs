use helpers::*;
#[test]
fn repeating_read_prefix() {
    // w(a).a(c).w(b).b(c).a(d).b(e).() !w[x].!x[y].()
    let mut pipl = Pipl::new();
    names!(|pipl| { w x y a b c d e });
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    builder
        .send(w).names(&[a]).call(log("w(a)", actual))
        .send(a).names(&[c]).call(log("a(c)", actual))
        .send(w).names(&[b]).call(log("w(b)", actual))
        .send(b).names(&[c]).call(log("b(c)", actual))
        .send(a).names(&[d]).call(log("a(d)", actual))
        .send(b).names(&[e]).call(log("b(e)", actual));
    builder
        .read(w).names(&[x]).repeat().call(log("!w[x]", actual))
        .read(x).names(&[y]).repeat().call(log("!x[y]", actual));
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs_empty = &mut Refs::new();
    let refs_wx1 = &mut Refs::new();
    let refs_wx2 = &mut Refs::new();
    // w(a).a(c) !w[x].!x[y]
    expected.log("w(a)", refs_empty.clone());
    expected.log("a(c)", refs_empty.clone());
    refs_wx1.insert(x.clone(), a.clone());
    expected.log("!w[x]", refs_wx1.clone());
    refs_wx1.insert(y.clone(), c.clone());
    expected.log("!x[y]", refs_wx1.clone());
    pipl.step();
    pipl.step();
    // w(b).b(c) !w[x].!x[y]
    expected.log("w(b)", refs_empty.clone());
    expected.log("b(c)", refs_empty.clone());
    refs_wx2.insert(x.clone(), b.clone());
    expected.log("!w[x]", refs_wx2.clone());
    refs_wx2.insert(y.clone(), c.clone());
    expected.log("!x[y]", refs_wx2.clone());
    pipl.step();
    pipl.step();
    // a(d).b(e) !x[y].()
    expected.log("a(d)", refs_empty.clone());
    expected.log("b(e)", refs_empty.clone());
    refs_wx1.insert(y.clone(), d.clone());
    expected.log("!x[y]", refs_wx1.clone());
    refs_wx2.insert(y.clone(), e.clone());
    expected.log("!x[y]", refs_wx2.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
}
