use helpers::*;
#[test]
fn polyadic() {
    // w[x,y].() w(a,b).()
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b) = (&n("a"), &n("b"));
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    builder.read(w).names(&[x, y]).call(log("w[x,y]", actual));
    builder.send(w).names(&[a, b]).call(log("w(a,b)", actual));
    let mut pipl = Pipl::new();
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs_empty = Refs::new();
    let refs_read = &mut Refs::new();
    refs_read.set(x.clone(), a.clone());
    refs_read.set(y.clone(), b.clone());
    expected.log("w[x,y]", refs_read.clone());
    expected.log("w(a,b)", refs_empty.clone());
    pipl.step();
    assert_eq_results(actual, expected);
}
