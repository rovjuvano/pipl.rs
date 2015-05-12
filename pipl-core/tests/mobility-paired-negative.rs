extern crate pipl;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use pipl::Atom;
use pipl::Pipl;

type Results = Rc<RefCell<HashMap<&'static str, Atom>>>;

// w[m].m(n).() w[o].o(p).() w(z).z[z].()

fn w_send(pipl: &mut Pipl, w: Atom, n1: &'static str, n2: &'static str, results: Results) {
    pipl.add_negative(w, move |pipl, args| {
        let a = args[0];
        results.borrow_mut().insert(n1, a);
        let results = results.clone();
        pipl.add_positive(a, move |pipl, args| {
            let b = pipl.atom();
            args.push(b);
            results.borrow_mut().insert(n2, b);
        });
    });
}
fn wm_mn(pipl: &mut Pipl, w: Atom, results: Results) {
    w_send(pipl, w, "m", "n", results);
}
fn wo_op(pipl: &mut Pipl, w: Atom, results: Results) {
    w_send(pipl, w, "o", "p", results);
}
fn wz_zz(pipl: &mut Pipl, w: Atom, results: Results) {
    pipl.add_positive(w, move |pipl, args| {
        let z = pipl.atom();
        args.push(z);
        results.borrow_mut().insert("z0", z);
        let results = results.clone();
        pipl.add_negative(z, move |_pipl, args| {
            results.borrow_mut().insert("z1", args[0]);
        });
    });
}

fn run<T>(func: T)
where T: Fn(&mut Pipl, Atom, Results) {
    Pipl::connect(|pipl| {
        let results = Rc::new(RefCell::new(HashMap::new()));
        let w = pipl.atom();
        func(pipl, w, results.clone());
        let keys = { let mut t = results.borrow().keys().map(|k| k.clone()).collect::<Vec<&str>>(); t.sort(); t };
        let expected_keys = if keys.contains(&"m") { vec!["m", "n", "z0", "z1"] } else { vec!["o", "p", "z0", "z1"] };
        assert_eq!(expected_keys, keys);
        assert_eq!(results.borrow().get(keys[0]), results.borrow().get("z0"));
        assert_eq!(results.borrow().get(keys[1]), results.borrow().get("z1"));
    });
}

#[test]
fn moz() {
    run(|pipl, w, results| {
        wm_mn(pipl, w, results.clone());
        wo_op(pipl, w, results.clone());
        wz_zz(pipl, w, results.clone());
    });
}
#[test]
fn mzo() {
    run(|pipl, w, results| {
        wm_mn(pipl, w, results.clone());
        wz_zz(pipl, w, results.clone());
        wo_op(pipl, w, results.clone());
    });
}
#[test]
fn omz() {
    run(|pipl, w, results| {
        wo_op(pipl, w, results.clone());
        wm_mn(pipl, w, results.clone());
        wz_zz(pipl, w, results.clone());
    });
}
#[test]
fn ozm() {
    run(|pipl, w, results| {
        wo_op(pipl, w, results.clone());
        wz_zz(pipl, w, results.clone());
        wm_mn(pipl, w, results.clone());
    });
}
#[test]
fn zmo() {
    run(|pipl, w, results| {
        wz_zz(pipl, w, results.clone());
        wm_mn(pipl, w, results.clone());
        wo_op(pipl, w, results.clone());
    });
}
#[test]
fn zom() {
    run(|pipl, w, results| {
        wz_zz(pipl, w, results.clone());
        wo_op(pipl, w, results.clone());
        wm_mn(pipl, w, results.clone());
    });
}
