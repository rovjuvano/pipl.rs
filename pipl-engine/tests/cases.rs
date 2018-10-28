extern crate pipl_engine;
// use pipl_engine::builders::inverted::Builder;
use pipl_engine::Pipl2;
use std::cell::RefCell;
use std::rc::Rc;
#[test]
#[ignore]
fn simplest_reaction() {
    // w[x] w(a)
    let pipl = &mut Pipl2::new();
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
    let pipl = &mut Pipl2::new();
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
// #[test]
// fn swx_rxy__rwz_szz() {
//     // w(x).x[y].() w[z].z(z).()
//     let pipl = &mut Pipl2::new();
//     let w = pipl.atom("w");
//     let x = pipl.atom("x");
//     let y = pipl.atom("y");
//     let z = pipl.atom("z");
//     let rwz = pipl.read(w, &[&x]);
//     let swx = pipl.send(w, &[z]);
//     let rxy = pipl.read(x, &[y]);
//     let szz = pipl.send(z, &[z]);
//     pipl.reaction(rwz, szz);
//     pipl.reaction(swx, rxy);
//     pipl.excite(rwz);
//     pipl.excite(swx);
//     println!("{:#?}", pipl);
// }
// #[test]
// #[ignore]
// fn simplest_reaction() {
//     // w[x] w(a)
//     let pipl = &mut Pipl2::new();
//     let builder = &mut Builder::new();
//     let a = pipl.atom("a"); // 0
//     let b = pipl.atom("b"); // 1
//     let c = pipl.atom("c"); // 2
//     let d = pipl.atom("d"); // 3
//     let e = pipl.atom("e"); // 4
//     let f = pipl.atom("f"); // 5
//     let mut p = Builder::parallel();
//     Builder::sequence().send(d).read(c).parallel(&mut p);
//     Builder::sequence().read(d).send(c).parallel(&mut p);
//     let mut p2 = Builder::parallel();
//     Builder::sequence().send(f).read(e).parallel(&mut p2);
//     Builder::sequence().read(f).send(e).parallel(&mut p2);
//     p2.prefix().send(d).read(c).parallel(&mut p);
//     p.prefix().send(b).read(a).launch(builder);
//     let mut ch = Builder::choice();
//     Builder::sequence().send(d).read(c).choice(&mut ch);
//     Builder::sequence().read(d).send(c).choice(&mut ch);
//     ch.prefix().send(b).read(a).launch(builder);
//     let s = Builder::sequence().send(f);
//     let ss = s.send(f);
//     ss.send(f).launch(builder);
//     Builder::sequence().send(a).read(b).launch(builder);
//     println!("{:#?}", builder);
//     assert_eq!(true, false);
// }
/*
use helpers::*;
#[test]
fn simplest_reaction() {
    // w[x] w(a)
    let mut pipl = Pipl::new();
    names!(|pipl| { w x a });
    let actual = &Rc::new(Results::new());
    let mut builder = PiplBuilder::new();
    builder.read(w).names(&[x]).call(log("w[x]", actual));
    builder.send(w).names(&[a]).call(log("w(a)", actual));
    builder.apply(&mut pipl);
    let expected = &Rc::new(Results::new());
    let refs = &mut Refs::new();
    expected.log("w(a)", refs.clone());
    refs.set(x.clone(), a.clone());
    expected.log("w[x]", refs.clone());
    pipl.step();
    assert_eq_results(&pipl, actual, expected);
}
*/
