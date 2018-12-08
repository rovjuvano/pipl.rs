extern crate pipl_engine;
use pipl_engine::{Call, CallFrame, Name, Pipl, PiplBuilder};
use std::env;
type N = String;
#[derive(Debug)]
struct EchoCall {
    name: Name,
}
impl EchoCall {
    fn new(name: Name) -> Self {
        EchoCall { name: name }
    }
}
impl Call for EchoCall {
    fn call(&self, frame: CallFrame) {
        println!("{}", frame.get_value::<N>(&self.name).unwrap());
    }
}
fn make_read(pipl: &mut Pipl, builder: &mut PiplBuilder, echo: &Name) {
    let arg = pipl.new_name("".to_owned());
    builder
        .read(echo).names(&[&arg]).repeat()
        .call(EchoCall::new(arg));
}
fn make_send(pipl: &mut Pipl, builder: &mut PiplBuilder, echo: &Name, arg: String) {
    let name = pipl.new_name(arg);
    builder.send(echo).names(&[&name]);
}
fn main() {
    let mut pipl = Pipl::new();
    let mut builder = PiplBuilder::new();
    let echo = &pipl.new_name("echo".to_owned());
    make_read(&mut pipl, &mut builder, echo);
    for arg in env::args().skip(1) {
        make_send(&mut pipl, &mut builder, echo, arg);
        builder.apply(&mut pipl);
        pipl.step();
        pipl.step();
    }
}
