use helpers::*;
#[test]
fn new_names_in_repeating_read() {
    // w[x].w[y].y(b).x(c).() ![a]z[].w(a).a[x].() !a(d).() z().z().()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let (a, b, c, d) = (&n("a"), &n("b"), &n("c"), &n("d"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![read(w, &[x]), read(w, &[y]), send(y, &[b]), send(x, &[c])], Terminal, actual.clone()));
    pipl.add(make(vec![read_many(z, &[]).new_names(&[a]), send(w, &[a]), read(a, &[x])], Terminal, actual.clone()));
    pipl.add(make(vec![send_many(a, &[d])], Terminal, actual.clone()));
    pipl.add(make(vec![send(z, &[]), send(z, &[])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = Refs::new();
    let refs_wx = &mut Refs::new();
    let refs_wax = &mut Refs::new();
    let refs_way = &mut Refs::new();
    refs_wax.set(a.clone(), a.dup());
    refs_way.set(a.clone(), a.dup());
    expected.log(f(&read_many(z, &[])), refs_wax.clone());
    expected.log(f(&read_many(z, &[])), refs_way.clone());
    expected.log(f(&send(z, &[])), refs_empty.clone());
    expected.log(f(&send(z, &[])), refs_empty.clone());
    pipl.step();
    pipl.step();
    refs_wx.set(x.clone(), a.dup());
    expected.log(f(&read(w, &[x])), refs_wx.clone());
    expected.log(f(&send(w, &[a])), refs_wax.clone());
    pipl.step();
    refs_wx.set(y.clone(), a.dup());
    expected.log(f(&read(w, &[y])), refs_wx.clone());
    expected.log(f(&send(w, &[a])), refs_way.clone());
    pipl.step();
    refs_way.set(x.clone(), b.clone());
    expected.log(f(&send(y, &[b])), refs_wx.clone());
    expected.log(f(&read(a, &[x])), refs_way.clone());
    pipl.step();
    refs_wax.set(x.clone(), c.clone());
    expected.log(f(&send(x, &[c])), refs_wx.clone());
    expected.log(f(&read(a, &[x])), refs_wax.clone());
    pipl.step();
    assert_eq_results(actual, expected);
}
