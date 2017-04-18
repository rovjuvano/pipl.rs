extern crate pipl_engine;
use pipl_engine::{Call, Name, Pipl, Prefix, Process, Refs, Sequence};
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
fn make_read(echo: Name) -> Sequence {
    let arg = n(());
    Sequence::new(
        vec![],
        Prefix::read_many(echo, vec![arg.clone()]),
        Process::new_call(Rc::new(EchoCall::new(arg)), Process::Terminal)
    )
}
fn make_send(echo: Name, arg: String) -> Sequence {
    Sequence::new(
        vec![],
        Prefix::send(echo.clone(), vec![n(arg)]),
        Process::Terminal
    )
}
fn main() {
    let mut pipl = Pipl::new();
    let echo = &n("echo");
    pipl.add(make_read(echo.clone()));
    for arg in env::args().skip(1) {
        pipl.add(make_send(echo.clone(), arg));
        pipl.step();
        pipl.step();
    }
}
