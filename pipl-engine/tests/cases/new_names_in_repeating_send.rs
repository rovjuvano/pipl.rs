use helpers::*;
#[test]
fn new_names_in_repeating_send() {
    // w[x].w[y].y(b).x(c).() ![a]w(a).a[x].() !a(d).()
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b, c, d) = (&n("a"), &n("b"), &n("c"), &n("d"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![read(w, &[x]), read(w, &[y]), send(y, &[b]), send(x, &[c])], Terminal, actual.clone()));
    pipl.add(make(vec![send_many(w, &[a]).new_names(&[a]), read(a, &[x])], Terminal, actual.clone()));
    pipl.add(make(vec![send_many(a, &[d])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_wx = &mut Refs::new();
    let refs_wax = &mut Refs::new();
    let refs_way = &mut Refs::new();
    refs_wx.set(x.clone(), a.dup());
    refs_wax.set(a.clone(), a.dup());
    expected.log(f(&read(w, &[x])), refs_wx.clone());
    expected.log(f(&send_many(w, &[a])), refs_wax.clone());
    pipl.step();
    refs_wx.set(y.clone(), a.dup());
    refs_way.set(a.clone(), a.dup());
    expected.log(f(&read(w, &[y])), refs_wx.clone());
    expected.log(f(&send_many(w, &[a])), refs_way.clone());
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
