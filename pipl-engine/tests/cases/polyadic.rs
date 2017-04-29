use helpers::*;
#[test]
fn polyadic() {
    // w[x,y].() w(a,b).()
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b) = (&n("a"), &n("b"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![read(w, &[x, y])], Terminal, actual.clone()));
    pipl.add(make(vec![send(w, &[a, b])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = Refs::new();
    let refs_read = &mut Refs::new();
    refs_read.set(x.clone(), a.clone());
    refs_read.set(y.clone(), b.clone());
    expected.log(f(&read(w, &[x, y])), refs_read.clone());
    expected.log(f(&send(w, &[a, b])), refs_empty.clone());
    pipl.step();
    assert_eq_results(actual, expected);
}
