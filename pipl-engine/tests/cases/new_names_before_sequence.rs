use helpers::*;
#[test]
fn new_names_before_sequence() {
    // w[x].[w, x].y(w).a(b).x(c).() w(a).y[m].a[n].x[o].()
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b, c) = (&n("a"), &n("b"), &n("c"));
    let (m, n, o) = (&n("m"), &n("n"), &n("o"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(
        vec![read(w, &[x])],
        new_names(&[w, x],
            make_p(vec![send(y, &[w]), send(a, &[b]), send(x, &[c])], Terminal, actual.clone()),
        ),
        actual.clone()
    ));
    pipl.add(make(vec![send(w, &[a]), read(y, &[m]), read(a, &[n]), read(x, &[o])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs_wx.clone());
    expected.log(f(&send(w, &[a])), refs_wa.clone());
    pipl.step();
    let w2 = &w.dup();
    refs_wx.set(w.clone(), w2.clone());
    refs_wx.set(x.clone(), x.dup());
    refs_wa.set(m.clone(), w2.clone());
    expected.log(f(&read(y, &[m])), refs_wa.clone());
    expected.log(f(&send(y, &[w])), refs_wx.clone());
    pipl.step();
    refs_wa.set(n.clone(), b.clone());
    expected.log(f(&read(a, &[n])), refs_wa.clone());
    expected.log(f(&send(a, &[b])), refs_wx.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual.clone(), expected);
    assert_ne_names(&actual.get(&f(&send(y, &[w]))).get(0).unwrap().get(w), w);
    assert_ne_names(&actual.get(&f(&read(y, &[m]))).get(0).unwrap().get(m), w);
}
