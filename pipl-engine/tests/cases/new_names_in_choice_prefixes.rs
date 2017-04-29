use helpers::*;
#[test]
fn new_names_in_choice_prefixes() {
    // w[x].(+ [m]w[y].y(m).m(b).() )
    // w(a).(+ [n]w(n).n[o].o[p].() )
    // m[z].() n(c).() o(d).()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let (a, b, c, d) = (&n("a"), &n("b"), &n("c"), &n("d"));
    let (m, n, o, p) = (&n("m"), &n("n"), &n("o"), &n("p"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(
        vec![read(w, &[x])],
        choice(vec![
            make(vec![read(w, &[y]).new_names(&[m]), send(y, &[m]), send(m, &[b])],Terminal, actual.clone()),
        ]),
        actual.clone()
    ));
    pipl.add(make(
        vec![send(w, &[a])],
        choice(vec![
            make(vec![send(w, &[n]).new_names(&[n]), read(n, &[o]), read(o, &[p])],Terminal, actual.clone()),
        ]),
        actual.clone()
    ));
    pipl.add(make(vec![read(m, &[z])], Terminal, actual.clone()));
    pipl.add(make(vec![send(n, &[c])], Terminal, actual.clone()));
    pipl.add(make(vec![send(o, &[d])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wa = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log(f(&read(w, &[x])), refs_wx.clone());
    expected.log(f(&send(w, &[a])), refs_wa.clone());
    pipl.step();
    let m2 = m.dup();
    let n2 = n.dup();
    refs_wx.set(m.clone(), m2.clone());
    refs_wx.set(y.clone(), n2.clone());
    refs_wa.set(n.clone(), n2.clone());
    expected.log(f(&read(w, &[y])), refs_wx.clone());
    expected.log(f(&send(w, &[n])), refs_wa.clone());
    pipl.step();
    refs_wa.set(o.clone(), m2.clone());
    expected.log(f(&read(n, &[o])), refs_wa.clone());
    expected.log(f(&send(y, &[m])), refs_wx.clone());
    pipl.step();
    refs_wa.set(p.clone(), b.clone());
    expected.log(f(&read(o, &[p])), refs_wa.clone());
    expected.log(f(&send(m, &[b])), refs_wx.clone());
    pipl.step();
    assert_eq_results(actual.clone(), expected);
}
