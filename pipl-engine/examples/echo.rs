extern crate pipl_engine;
use pipl_engine::{Call, CallFrame, Name, Pipl, PiplBuilder};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::env;
use std::rc::Rc;
#[derive(Clone, Debug)]
struct NameValues {
    map: Rc<RefCell<BTreeMap<Name, String>>>,
}
impl NameValues {
    fn new() -> Self {
        NameValues {
            map: Rc::new(RefCell::new(BTreeMap::new())),
        }
    }
    fn name<S: Into<String>>(&mut self, pipl: &mut Pipl, value: S) -> Name {
        let name = pipl.new_name();
        self.map.borrow_mut().insert(name, value.into());
        name
    }
    fn get(&self, name: &Name) -> Option<String> {
        // ** bad clone ** //
        self.map.borrow().get(&name).cloned()
    }
}
#[derive(Debug)]
struct EchoCall {
    name: Name,
    values: NameValues,
}
impl EchoCall {
    fn new(name: Name, values: NameValues) -> Self {
        EchoCall { name, values }
    }
}
impl Call for EchoCall {
    fn call(&self, frame: CallFrame) {
        let name = frame.get_name(&self.name);
        println!("{}", self.values.get(&name).unwrap());
    }
}
fn make_read(pipl: &mut Pipl, builder: &mut PiplBuilder, values: &mut NameValues, echo: &Name) {
    let arg = pipl.new_name();
    builder
        .read(echo).names(&[&arg]).repeat()
        .call(EchoCall::new(arg, values.clone()));
}
fn make_send(
    pipl: &mut Pipl,
    builder: &mut PiplBuilder,
    values: &mut NameValues,
    echo: &Name,
    arg: String,
) {
    let name = values.name(pipl, arg);
    builder.send(echo).names(&[&name]);
}
fn main() {
    let mut pipl = Pipl::new();
    let mut builder = PiplBuilder::new();
    let mut values = NameValues::new();
    let echo = &values.name(&mut pipl, "echo");
    make_read(&mut pipl, &mut builder, &mut values, echo);
    for arg in env::args().skip(1) {
        make_send(&mut pipl, &mut builder, &mut values, echo, arg);
        builder.apply(&mut pipl);
        pipl.step();
        pipl.step();
    }
}
