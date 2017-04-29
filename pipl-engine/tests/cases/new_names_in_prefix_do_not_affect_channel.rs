use helpers::*;
#[test]
fn new_names_in_prefix_do_not_affect_channel() {
    // [w]w[m].(+ [x]x[n].(| [y]y[o].() ) )
    // [w]w(a).(+ [x]x(b).(| [y]y(c).() ) )
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b, c) = (&n("a"), &n("b"), &n("c"));
    let (m, n, o) = (&n("m"), &n("n"), &n("o"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![read(w, &[m]).new_names(&[w])], choice(vec![
        make(vec![read(x, &[n]).new_names(&[x])], parallel(vec![
                make(vec![read(y, &[o]).new_names(&[y])], Terminal, actual.clone())
            ]),
            actual.clone()
        )]),
        actual.clone()
    ));
    pipl.add(make(vec![send(w, &[a]).new_names(&[w])], choice(vec![
            make(vec![send(x, &[b]).new_names(&[x])], parallel(vec![
                make(vec![send(y, &[c]).new_names(&[y])], Terminal, actual.clone())
            ]),
            actual.clone()
        )]),
        actual.clone()
    ));
    let expected = Rc::new(Results::new());
    let refs_wm = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    refs_wm.set(w.clone(), w.dup());
    refs_wm.set(m.clone(), a.clone());
    refs_wa.set(w.clone(), w.dup());
    expected.log(f(&read(w, &[m])), refs_wm.clone());
    expected.log(f(&send(w, &[a])), refs_wa.clone());
    pipl.step();
    refs_wm.set(x.clone(), x.dup());
    refs_wm.set(n.clone(), b.clone());
    refs_wa.set(x.clone(), x.dup());
    expected.log(f(&read(x, &[n])), refs_wm.clone());
    expected.log(f(&send(x, &[b])), refs_wa.clone());
    pipl.step();
    refs_wm.set(y.clone(), y.dup());
    refs_wm.set(o.clone(), c.clone());
    refs_wa.set(y.clone(), y.dup());
    expected.log(f(&read(y, &[o])), refs_wm.clone());
    expected.log(f(&send(y, &[c])), refs_wa.clone());
    pipl.step();
    // assert_eq_results(actual.clone(), expected);
    let refs_yo = actual.get(&f(&read(y, &[o]))).get(0).unwrap().clone();
    let refs_yc = actual.get(&f(&send(y, &[c]))).get(0).unwrap().clone();
    assert_ne_names(&refs_yo.get(w), w);
    assert_ne_names(&refs_yc.get(w), w);
    assert_ne_names(&refs_yo.get(x), x);
    assert_ne_names(&refs_yc.get(x), x);
    assert_ne_names(&refs_yo.get(y), y);
    assert_ne_names(&refs_yc.get(y), y);
}
