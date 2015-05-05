extern crate pipl;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use pipl::Atom;
use pipl::Pipl;

fn wx_xy(pipl: &mut Pipl, w: Atom, results: Rc<RefCell<HashMap<&'static str, Atom>>>) {
    pipl.add_positive(w, move |pipl, args| {
        let x = pipl.atom();
        args.push(x);
        results.borrow_mut().insert("x", x);
        let results = results.clone();
        pipl.add_negative(x, move |_pipl, args| {
            let y = args[0];
            results.borrow_mut().insert("y", y);
        });
    });
}
fn wz_zz(pipl: &mut Pipl, w: Atom, _results: Rc<RefCell<HashMap<&'static str, Atom>>>) {
    pipl.add_negative(w, move |pipl, args| {
        let z = args[0];
        pipl.add_positive(z, move |_pipl, args| {
            args.push(z);
        });
    });
}

#[test]
// w(x).x[y].() w[z].z(z).()
fn positive_reacts_with_negative() {
    Pipl::connect(|pipl| {
        let results = Rc::new(RefCell::new(HashMap::new()));
        let w = pipl.atom();
        wx_xy(pipl, w, results.clone());
        wz_zz(pipl, w, results.clone());
        let keys = {let mut t = results.borrow().keys().map(|k| k.clone()).collect::<Vec<&str>>(); t.sort(); t };
        assert_eq!(vec!["x", "y"], keys);
        assert_eq!(results.borrow().get("x"), results.borrow().get("y"));
    });
}
#[test]
fn negative_reacts_with_positive() {
    Pipl::connect(|pipl| {
        let results = Rc::new(RefCell::new(HashMap::new()));
        let w = pipl.atom();
        wz_zz(pipl, w, results.clone());
        wx_xy(pipl, w, results.clone());
        let keys = {let mut t = results.borrow().keys().map(|k| k.clone()).collect::<Vec<&str>>(); t.sort(); t };
        assert_eq!(vec!["x", "y"], keys);
        assert_eq!(results.borrow().get("x"), results.borrow().get("y"));
    });
}
