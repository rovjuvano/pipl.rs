extern crate pipl;

#[test]
fn it_works() {
    let mut pipl = pipl::connect();
    assert_eq!("Atom(0)", format!("{:?}", pipl.atom()));
}
