use helpers::*;
#[test]
fn new_names_in_parallel_prefixes() {
    // w[x].(| [x]w(x).x(b).() [a]w[y].a(c).() x[z].() ) w(a).a[z].()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let (a, b, c) = (&n("a"), &n("b"), &n("c"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(
        vec![read(w, &[x])],
        parallel(vec![
            make(vec![send(w, &[x]).new_names(&[x]), send(x, &[b])], Terminal, actual.clone()),
            make(vec![read(w, &[y]).new_names(&[a]), send(a, &[c])], Terminal, actual.clone()),
            make(vec![read(x, &[z])], Terminal, actual.clone()),
        ]),
        actual.clone()
    ));
    pipl.add(make(vec![send(w, &[a]), read(a, &[z])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs_wx.clone());
    expected.log(f(&send(w, &[a])), refs_wa.clone());
    pipl.step();
    let refs_wxwx = &mut refs_wx.clone();
    let refs_wxwy = &mut refs_wx.clone();
    let x2 = x.dup();
    refs_wxwx.set(x.clone(), x2.clone());
    refs_wxwy.set(y.clone(), x2.clone());
    refs_wxwy.set(a.clone(), a.dup());
    expected.log(f(&read(w, &[y])), refs_wxwy.clone());
    expected.log(f(&send(w, &[x])), refs_wxwx.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual.clone(), expected);
    assert_ne_names(&actual.get(&f(&read(w, &[y]))).get(0).unwrap().get(a), a);
    assert_ne_names(&actual.get(&f(&send(w, &[x]))).get(0).unwrap().get(x), x);
    assert_eq!(
        &actual.get(&f(&read(w, &[y]))).get(0).unwrap().get(y),
        &actual.get(&f(&send(w, &[x]))).get(0).unwrap().get(x)
    );
}
