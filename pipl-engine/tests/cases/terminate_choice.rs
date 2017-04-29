use helpers::*;
#[test]
fn terminate_choice() {
    // w[x].(+ x[y].() y[z].y[z].() ) w(a).a(b).y(c).() b(d).()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let (a, b, c, d) = (&n("a"), &n("b"), &n("c"), &n("d"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(
        vec![read(w, &[x])],
        choice(vec![
            make(vec![read(x, &[y])], Terminal, actual.clone()),
            make(vec![read(y, &[z]), read(y, &[z])], Terminal, actual.clone()),
        ]),
        actual.clone()
    ));
    pipl.add(make(vec![send(w, &[a]), send(a, &[b]), send(y, &[c])], Terminal, actual.clone()));
    pipl.add(make(vec![send(b, &[d])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = Refs::new();
    let refs_wx = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs_wx.clone());
    let refs_wxxy = &mut refs_wx.clone();
    refs_wxxy.set(y.clone(), b.clone());
    expected.log(f(&read(x, &[y])), refs_wxxy.clone());
    expected.log(f(&send(w, &[a])), refs_empty.clone());
    expected.log(f(&send(a, &[b])), refs_empty.clone());
    pipl.step();
    pipl.step();
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
