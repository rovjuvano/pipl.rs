extern crate pipl_engine;
use pipl_engine::Pipl;
use std::cell::RefCell;
use std::rc::Rc;
#[test]
#[ignore]
fn simplest_reaction() {
    // w[x] w(a)
    let pipl = &mut Pipl::new();
    let a = pipl.atom("a");
    let w = pipl.atom("w");
    let x = pipl.atom("x");
    let t = pipl.terminal();
    let results = Rc::new(RefCell::new(Vec::new()));
    let rwx_call = {
        let r = results.clone();
        pipl.call(move || r.borrow_mut().push(2), t)
    };
    let swa_call = {
        let r = results.clone();
        pipl.call(move || r.borrow_mut().push(1), t)
    };
    let rwx = pipl.read(w, &[x], rwx_call);
    let swa = pipl.send(w, &[a], swa_call);
    pipl.excite(rwx);
    pipl.excite(swa);
    pipl.step();
    println!("{:#?}", pipl);
    assert_eq!(results.replace(Vec::new()), vec![1, 2]);
}
#[test]
fn simplest_mobility() {
    // w(x).x[y].() w[z].z(z).()
    let pipl = &mut Pipl::new();
    let w = pipl.atom("0");
    let x = pipl.atom("1");
    let y = pipl.atom("2");
    let z = pipl.atom("3");
    let t = pipl.terminal();
    let next = pipl.read(x, &[y], t);
    let send = pipl.send(w, &[x], next);
    let next = pipl.send(z, &[z], t);
    let read = pipl.read(w, &[z], next);
    pipl.excite(send);
    pipl.excite(read);
    pipl.step();
    // pipl.step();
    println!("{:#?}", pipl);
}
