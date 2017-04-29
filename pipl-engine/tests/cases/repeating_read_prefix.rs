use helpers::*;
#[test]
fn repeating_read_prefix() {
    // w(a).a(c).w(b).b(c).a(d).b(e).() !w[x].!x[y].()
    let (w, x, y) = (&n("w"), &n("x"), &n("y"));
    let (a, b, c, d, e) = (&n("a"), &n("b"), &n("c"), &n("d"), &n("e"));
    let mut pipl = Pipl::new();
    let actual = Rc::new(Results::new());
    pipl.add(make(vec![
            send(w, &[a]), send(a, &[c]),
            send(w, &[b]), send(b, &[c]),
            send(a, &[d]), send(b, &[e]),
        ],
        Terminal, actual.clone())
    );
    pipl.add(make(vec![read_many(w, &[x]), read_many(x, &[y])], Terminal, actual.clone()));
    let expected = Rc::new(Results::new());
    let refs_empty = &mut Refs::new();
    let refs_wx1 = &mut Refs::new();
    let refs_wx2 = &mut Refs::new();
    // w(a).a(c) !w[x].!x[y]
    expected.log(f(&send(w, &[a])), refs_empty.clone());
    expected.log(f(&send(a, &[c])), refs_empty.clone());
    refs_wx1.set(x.clone(), a.clone());
    expected.log(f(&read_many(w, &[x])), refs_wx1.clone());
    refs_wx1.set(y.clone(), c.clone());
    expected.log(f(&read_many(x, &[y])), refs_wx1.clone());
    pipl.step();
    pipl.step();
    // w(b).b(c) !w[x].!x[y]
    expected.log(f(&send(w, &[b])), refs_empty.clone());
    expected.log(f(&send(b, &[c])), refs_empty.clone());
    refs_wx2.set(x.clone(), b.clone());
    expected.log(f(&read_many(w, &[x])), refs_wx2.clone());
    refs_wx2.set(y.clone(), c.clone());
    expected.log(f(&read_many(x, &[y])), refs_wx2.clone());
    pipl.step();
    pipl.step();
    // a(d).b(e) !x[y].()
    expected.log(f(&send(a, &[d])), refs_empty.clone());
    expected.log(f(&send(b, &[e])), refs_empty.clone());
    refs_wx1.set(y.clone(), d.clone());
    expected.log(f(&read_many(x, &[y])), refs_wx1.clone());
    refs_wx2.set(y.clone(), e.clone());
    expected.log(f(&read_many(x, &[y])), refs_wx2.clone());
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
