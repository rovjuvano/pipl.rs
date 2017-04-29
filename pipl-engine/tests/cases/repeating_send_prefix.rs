use helpers::*;
#[test]
fn repeating_send_prefix() {
    // w[a].a[c].w[b].b[c].a[d].b[e].() !w(x).!x(y).()
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b, c, d, e) = (&n("a"), &n("b"), &n("c"), &n("d"), &n("e"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![
            read(w, &[a]), read(a, &[c]),
            read(w, &[b]), read(b, &[c]),
            read(a, &[d]), read(b, &[e]),
        ],
        Terminal, actual.clone())
    );
    pipl.add(make(vec![send_many(w, &[x]), send_many(x, &[y])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = Refs::new();
    let refs_wa = &mut Refs::new();
    // w[a].a[c] !w(x).!x(y)
    refs_wa.set(a.clone(), x.clone());
    expected.log(f(&read(w, &[a])), refs_wa.clone());
    refs_wa.set(c.clone(), y.clone());
    expected.log(f(&read(a, &[c])), refs_wa.clone());
    expected.log(f(&send_many(w, &[x])), refs_empty.clone());
    expected.log(f(&send_many(x, &[y])), refs_empty.clone());
    pipl.step();
    pipl.step();
    // w[b].b[c] !w(x).!x(y)
    refs_wa.set(b.clone(), x.clone());
    expected.log(f(&read(w, &[b])), refs_wa.clone());
    refs_wa.set(c.clone(), y.clone());
    expected.log(f(&read(b, &[c])), refs_wa.clone());
    expected.log(f(&send_many(w, &[x])), refs_empty.clone());
    expected.log(f(&send_many(x, &[y])), refs_empty.clone());
    pipl.step();
    pipl.step();
    // a[d].b[e] !x(y).()
    refs_wa.set(d.clone(), y.clone());
    expected.log(f(&read(a, &[d])), refs_wa.clone());
    refs_wa.set(e.clone(), y.clone());
    expected.log(f(&read(b, &[e])), refs_wa.clone());
    expected.log(f(&send_many(x, &[y])), refs_empty.clone());
    expected.log(f(&send_many(x, &[y])), refs_empty.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
