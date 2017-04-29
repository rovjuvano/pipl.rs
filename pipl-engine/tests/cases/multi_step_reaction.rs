use helpers::*;
#[test]
fn multi_step_reaction() {
    // w[x].w[y] w(a).w(b)
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b) = (&n("a"), &n("b"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![read(w, &[x]), read(w, &[y])], Terminal, actual.clone()));
    pipl.add(make(vec![send(w, &[a]), send(w, &[b])], Terminal, actual.clone()));
    pipl.step();
    pipl.step();
    let expected = Rc::new(Results::new());
    let refs = &mut Refs::new();
    expected.log(f(&send(w, &[a])), refs.clone());
    expected.log(f(&send(w, &[b])), refs.clone());
    refs.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs.clone());
    refs.set(y.clone(), b.clone());
    expected.log(f(&read(w, &[y])), refs.clone());
    assert_eq_results(actual, expected);
}
