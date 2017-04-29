extern crate pipl_engine;
use pipl_engine::{Call, Name, Pipl, PiplBuilder, Refs};
use std::env;
use std::fmt;
use std::rc::Rc;
fn n<T: fmt::Debug + 'static>(name: T) -> Name {
    Name::new(name)
}
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
    fn call(&self, refs: Refs) -> Refs {
        let s = refs.get(&self.name);
        if s.raw().is::<String>() {
            println!("{}", s.raw().downcast_ref::<String>().unwrap());
        }
        refs
    }
}
fn make_read(builder: &mut PiplBuilder, echo: Name) {
    let arg = n(());
    builder.read(echo)
        .names(&[arg.clone()])
        .repeat()
        .call(Rc::new(EchoCall::new(arg)));
}
fn make_send(builder: &mut PiplBuilder, echo: Name, arg: String) {
    builder.send(echo)
        .names(&[n(arg)]);
}
fn main() {
    let mut pipl = Pipl::new();
    let mut builder = PiplBuilder::new();
    let echo = &n("echo");
    make_read(&mut builder, echo.clone());
    for arg in env::args().skip(1) {
        make_send(&mut builder, echo.clone(), arg);
        builder.apply(&mut pipl);
        pipl.step();
        pipl.step();
    }
}
