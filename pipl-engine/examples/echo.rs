extern crate pipl_engine;
use pipl_engine::{Call, Name, Pipl, PiplBuilder, Refs};
use std::env;
use std::rc::Rc;
type N = String;
fn n(value: String) -> Name<N> {
    Name::new(value)
}
#[derive(Debug)]
struct EchoCall {
    name: Name<N>,
}
impl EchoCall {
    fn new(name: Name<N>) -> Self {
        EchoCall { name: name }
    }
}
impl Call<N> for EchoCall {
    fn call(&self, refs: Refs<N>) -> Refs<N> {
        let s = refs.get(&self.name);
        println!("{}", s.raw());
        refs
    }
}
fn make_read(builder: &mut PiplBuilder<N>, echo: &Name<N>) {
    let arg = n("".to_owned());
    builder.read(echo)
        .names(&[&arg])
        .repeat()
        .call(Rc::new(EchoCall::new(arg)));
}
fn make_send(builder: &mut PiplBuilder<N>, echo: &Name<N>, arg: String) {
    builder.send(echo)
        .names(&[&n(arg)]);
}
fn main() {
    let mut pipl = Pipl::new();
    let mut builder = PiplBuilder::new();
    let echo = &n("echo".to_owned());
    make_read(&mut builder, echo);
    for arg in env::args().skip(1) {
        make_send(&mut builder, echo, arg);
        builder.apply(&mut pipl);
        pipl.step();
        pipl.step();
    }
}
