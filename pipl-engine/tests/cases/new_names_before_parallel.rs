use helpers::*;
#[test]
fn new_names_before_parallel() {
    // w[x].[w, x](| w(c).() x[o].() y[p].x(p).x(p) )
    // w(a).w[m].() a[n].() x[o].() y(b).()
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b, c) = (&n("a"), &n("b"), &n("c"));
    let (m, n, o, p) = (&n("m"), &n("n"), &n("o"), &n("p"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(
        vec![read(w, &[x])],
        new_names(&[w, x],
            parallel(vec![
                make(vec![send(w, &[c])], Terminal, actual.clone()),
                make(vec![read(x, &[o])], Terminal, actual.clone()),
                make(vec![read(y, &[p]), send(x, &[p]), send(x, &[p])], Terminal, actual.clone()),
            ]),
        ),
        actual.clone()
    ));
    pipl.add(make(vec![send(w, &[a]), read(w, &[m])], Terminal, actual.clone()));
    pipl.add(make(vec![read(a, &[n])], Terminal, actual.clone()));
    pipl.add(make(vec![send(x, &[o])], Terminal, actual.clone()));
    pipl.add(make(vec![send(y, &[b])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    let refs_yb = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs_wx.clone());
    expected.log(f(&send(w, &[a])), refs_wa.clone());
    pipl.step();
    refs_wx.set(w.clone(), w.dup());
    refs_wx.set(x.clone(), x.dup());
    let refs_wxxo = &mut refs_wx.clone();
    let refs_wxyp = &mut refs_wx.clone();
    refs_wxyp.set(p.clone(), b.clone());
    expected.log(f(&read(y, &[p])), refs_wxyp.clone());
    expected.log(f(&send(y, &[b])), refs_yb.clone());
    pipl.step();
    refs_wxxo.set(o.clone(), b.clone());
    expected.log(f(&read(x, &[o])), refs_wxxo.clone());
    expected.log(f(&send(x, &[p])), refs_wxyp.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
