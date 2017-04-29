use helpers::*;
#[test]
fn simplest_mobility() {
    // w(x).x[y].() w[z].z(z).()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![send(w, &[x]), read(x, &[y])], Terminal, actual.clone()));
    pipl.add(make(vec![read(w, &[z]), send(z, &[z])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wz = &mut Refs::new();
    refs_wz.set(z.clone(), x.clone());
    expected.log(f(&send(w, &[x])), refs_wx.clone());
    expected.log(f(&read(w, &[z])), refs_wz.clone());
    pipl.step();
    assert_eq_results(actual.clone(), expected.clone());
    refs_wx.set(y.clone(), x.clone());
    expected.log(f(&read(x, &[y])), refs_wx.clone());
    expected.log(f(&send(z, &[z])), refs_wz.clone());
    pipl.step();
    assert_eq_results(actual, expected);
}
