extern crate pipl_engine;
use pipl_engine::Pipl;
use std::cell::RefCell;
use std::rc::Rc;
#[test]
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
    let wx = pipl.read(w, &[x]);
    let rwx = pipl.reaction(wx, rwx_call);
    let wa = pipl.send(w, &[a]);
    let swa = pipl.reaction(wa, swa_call);
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

    // A: w(x).x[y].() - raw Pipl API
    // let xy = pipl.read(x, &[y]);
    // let next = pipl.reaction(xy, t);
    // let next = pipl.sequence(next);
    // let wx = pipl.send(w, &[x]);
    // let send = pipl.reaction(wx, next);

    // B: w(x).x[y].() - Reactant#reaction
    // let xy = pipl.read(x, &[y]).reaction(t);
    // let next = pipl.sequence(xy);
    // let send = pipl.send(w, &[x]).reaction(next);

    // C: w(x).x[y].() - Pipl#chain with slice
    // let xy = pipl.read(x, &[y]);
    // let wx = pipl.send(w, &[x]);
    // let send = pipl.chain(&[wx, xy], t);
    // // -- OR -- let send = pipl.chain(&[&wx, &xy], t);

    // D: w(x).x[y].() - Pipl#chain with vec
    let seq = vec![pipl.send(w, &[x]), pipl.read(x, &[y])];
    let send = pipl.chain(seq, t);

    let zz = pipl.send(z, &[z]);
    let next = pipl.reaction(zz, t);
    let next = pipl.sequence(next);
    let wz = pipl.read(w, &[z]);
    let read = pipl.reaction(wz, next);
    pipl.excite(send);
    pipl.excite(read);
    pipl.step();
    // pipl.step();
    println!("{:#?}", pipl);
}
