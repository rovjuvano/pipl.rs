extern crate pipl;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use pipl::Atom;
use pipl::Pipl;

type Results = Rc<RefCell<HashMap<&'static str, Atom>>>;

// w(m).m[n].() w(o).o[p].() w[z].z(z).()

fn w_read(pipl: &mut Pipl, w: Atom, n1: &'static str, n2: &'static str, results: Results) {
    pipl.add_positive(w, move |pipl, args| {
        let a = pipl.atom();
        args.push(a);
        results.borrow_mut().insert(n1, a);
        let results = results.clone();
        pipl.add_negative(a, move |_pipl, args| {
            results.borrow_mut().insert(n2, args[0]);
        });
    });
}
fn wm_mn(pipl: &mut Pipl, w: Atom, results: Results) {
    w_read(pipl, w, "m", "n", results);
}
fn wo_op(pipl: &mut Pipl, w: Atom, results: Results) {
    w_read(pipl, w, "o", "p", results);
}
fn wz_zz(pipl: &mut Pipl, w: Atom, results: Results) {
    pipl.add_negative(w, move |pipl, args| {
        let z = args[0];
        results.borrow_mut().insert("z", z);
        pipl.add_positive(z, move |_pipl, args| {
            args.push(z);
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
        let expected_keys = if keys.contains(&"m") { vec!["m", "n", "z"] } else { vec!["o", "p", "z"] };
        assert_eq!(expected_keys, keys);
        assert_eq!(results.borrow().get("z"), results.borrow().get(keys[0]));
        assert_eq!(results.borrow().get("z"), results.borrow().get(keys[1]));
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
