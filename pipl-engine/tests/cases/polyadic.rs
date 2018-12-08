use helpers::*;
#[test]
fn polyadic() {
    // w[x,y].() w(a,b).()
    let mut pipl = Pipl::new();
    names!(|pipl| { w x y a b });
    let actual = &Results::new();
    let mut builder = PiplBuilder::new();
    builder.read(w).names(&[x, y]).call(log("w[x,y]", actual));
    builder.send(w).names(&[a, b]).call(log("w(a,b)", actual));
    builder.apply(&mut pipl);
    let expected = &Results::new();
    let refs_empty = Refs::new();
    let refs_read = &mut Refs::new();
    refs_read.insert(x.clone(), a.clone());
    refs_read.insert(y.clone(), b.clone());
    expected.log("w[x,y]", refs_read.clone());
    expected.log("w(a,b)", refs_empty.clone());
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
}
