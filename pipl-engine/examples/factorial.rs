extern crate pipl_engine;
use pipl_engine::{Call, CallFrame, Name, Pipl, PiplBuilder};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::rc::Rc;
#[derive(Debug, Clone)]
enum N {
    Str(&'static str),
    Usize(usize),
}
impl fmt::Display for N {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            N::Str(x) => write!(f, "{:?}", x),
            N::Usize(x) => write!(f, "{:?}", x),
        }
    }
}
#[derive(Clone, Debug)]
struct NameValues {
    map: BTreeMap<Name, N>,
}
impl NameValues {
    fn new() -> Self {
        NameValues {
            map: BTreeMap::new(),
        }
    }
    fn name(&mut self, name_source: &mut impl NameSource, value: N) -> Name {
        let name = name_source.name();
        self.map.insert(name, value);
        name
    }
    fn get(&self, frame: &CallFrame, name: &Name) -> Option<&N> {
        self.map.get(&frame.get_name(name))
    }
}
trait NameSource {
    fn name(&mut self) -> Name;
}
impl NameSource for Pipl {
    fn name(&mut self) -> Name {
        self.new_name()
    }
}
impl<'a> NameSource for CallFrame<'a> {
    fn name(&mut self) -> Name {
        self.new_name()
    }
}
fn add_factorial(
    pipl: &mut Pipl,
    builder: &mut PiplBuilder,
    values: Rc<RefCell<NameValues>>,
    greater_than: &Name,
    subtract: &Name,
    multiply: &Name,
) -> Name {
    let mut values = values.borrow_mut();
    let fact = values.name(pipl, N::Str("fact"));
    let x = &values.name(pipl, N::Str("x"));
    let out = &values.name(pipl, N::Str("out"));
    let gt2 = &values.name(pipl, N::Str("greater-than-two"));
    let le2 = &values.name(pipl, N::Str("two-or-less"));
    let (t1, t2, t3) = (
        &values.name(pipl, N::Str("t1")),
        &values.name(pipl, N::Str("t2")),
        &values.name(pipl, N::Str("t3")),
    );
    let x_minus_1 = &values.name(pipl, N::Str("x-1"));
    let factorial_x_minus_1 = &values.name(pipl, N::Str("!(x-1)"));
    let result = &values.name(pipl, N::Str("result"));
    let one = &values.name(pipl, N::Usize(1));
    let two = &values.name(pipl, N::Usize(2));
    let c = builder
        .read(&fact)
        .names(&[x, out])
        .repeat()
        .send(greater_than)
        .restrict(&[gt2, le2])
        .names(&[x, two, gt2, le2])
        .choice();
    c.read(le2).send(out).names(&[x]);
    let p = c.read(gt2).parallel().restrict(&[t1, t2, t3]);
    p.send(subtract).names(&[x, one, t1]);
    p.read(t1)
        .names(&[x_minus_1])
        .send(&fact)
        .names(&[x_minus_1, t2]);
    p.read(t2)
        .names(&[factorial_x_minus_1])
        .send(multiply)
        .names(&[x, factorial_x_minus_1, t3]);
    p.read(t3).names(&[result]).send(out).names(&[result]);
    fact
}
fn add_print(pipl: &mut Pipl, builder: &mut PiplBuilder, values: Rc<RefCell<NameValues>>) -> Name {
    struct PrintCall {
        name: Name,
        values: Rc<RefCell<NameValues>>,
    }
    impl Call for PrintCall {
        fn call(&self, frame: CallFrame) {
            println!("{}", self.values.borrow().get(&frame, &self.name).unwrap());
        }
    }
    impl fmt::Debug for PrintCall {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "PrintCall({:?})", self.name)
        }
    }
    let name = values.borrow_mut().name(pipl, N::Str("print"));
    let arg = values.borrow_mut().name(pipl, N::Str("arg"));
    builder
        .read(&name).names(&[&arg]).repeat()
        .call(PrintCall { name: arg, values });
    name
}
fn add_greater_than(pipl: &mut Pipl, builder: &mut PiplBuilder, values: Rc<RefCell<NameValues>>) -> Name {
    struct GreaterThanCall {
        a: Name,
        b: Name,
        gt: Name,
        lte: Name,
        out: Name,
        values: Rc<RefCell<NameValues>>,
    }
    impl Call for GreaterThanCall {
        fn call(&self, mut frame: CallFrame) {
            let maybe = match (
                self.values.borrow().get(&frame, &self.a),
                self.values.borrow().get(&frame, &self.b),
            ) {
                (Some(N::Usize(a)), Some(N::Usize(b))) => {
                    let x = if a > b { &self.gt } else { &self.lte };
                    Some(frame.get_name(x))
                }
                _ => None,
            };
            if let Some(result) = maybe {
                frame.set_name(self.out.clone(), result);
            }
        }
    }
    impl fmt::Debug for GreaterThanCall {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("GreaterThanCall")
                .field("a", &self.a)
                .field("b", &self.b)
                .field("gt", &self.gt)
                .field("lte", &self.lte)
                .field("->", &self.out)
                .finish()
        }
    }
    let name = values.borrow_mut().name(pipl, N::Str(">"));
    let a = &values.borrow_mut().name(pipl, N::Str("a"));
    let b = &values.borrow_mut().name(pipl, N::Str("b"));
    let gt = &values.borrow_mut().name(pipl, N::Str("gt"));
    let lte = &values.borrow_mut().name(pipl, N::Str("lte"));
    let out = &values.borrow_mut().name(pipl, N::Str("->"));
    let gt_call = GreaterThanCall {
        a: a.clone(),
        b: b.clone(),
        gt: gt.clone(),
        lte: lte.clone(),
        out: out.clone(),
        values,
    };
    builder
        .read(&name).names(&[a, b, gt, lte]).repeat()
        .call(gt_call)
        .send(out);
    name
}
struct BinaryOpCall {
    label: String,
    f: Box<Fn(usize, usize) -> usize>,
    a: Name,
    b: Name,
    out: Name,
    values: Rc<RefCell<NameValues>>,
}
impl fmt::Debug for BinaryOpCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(&format!("BinaryOpCall({:?})", self.label))
            .field("a", &self.a)
            .field("b", &self.b)
            .field("out", &self.out)
            .finish()
    }
}
impl Call for BinaryOpCall {
    fn call(&self, mut frame: CallFrame) {
        let maybe = match (
            self.values.borrow().get(&frame, &self.a),
            self.values.borrow().get(&frame, &self.b),
        ) {
            (Some(N::Usize(a)), Some(N::Usize(b))) => Some((self.f)(*a, *b)),
            _ => None,
        };
        if let Some(result) = maybe {
            let x = self.values.borrow_mut().name(&mut frame, N::Usize(result));
            frame.set_name(self.out.clone(), x);
        }
    }
}
fn add_binary_op<T>(
    pipl: &mut Pipl,
    builder: &mut PiplBuilder,
    values: Rc<RefCell<NameValues>>,
    label: &'static str,
    f: T,
) -> Name
where
    T: Fn(usize, usize) -> usize + 'static,
{
    let name = values.borrow_mut().name(pipl, N::Str(label));
    let a = &values.borrow_mut().name(pipl, N::Str("a"));
    let b = &values.borrow_mut().name(pipl, N::Str("b"));
    let result = &values.borrow_mut().name(pipl, N::Str("="));
    let out = &values.borrow_mut().name(pipl, N::Str("->"));
    let op = BinaryOpCall {
        label: label.to_string(),
        f: Box::new(f),
        a: a.clone(),
        b: b.clone(),
        out: result.clone(),
        values,
    };
    builder
        .read(&name).names(&[a, b, out]).repeat()
        .call(op)
        .send(out).names(&[&result]);
    name
}
fn add_subtract(pipl: &mut Pipl, builder: &mut PiplBuilder, values: Rc<RefCell<NameValues>>) -> Name {
    add_binary_op(pipl, builder, values, "-", |a, b| a - b)
}
fn add_multiply(pipl: &mut Pipl, builder: &mut PiplBuilder, values: Rc<RefCell<NameValues>>) -> Name {
    add_binary_op(pipl, builder, values, "*", |a, b| a * b)
}
fn main() {
    let pipl = &mut Pipl::new();
    let builder = &mut PiplBuilder::new();
    let values = Rc::new(RefCell::new(NameValues::new()));
    let greater_than = &add_greater_than(pipl, builder, values.clone());
    let subtract = &add_subtract(pipl, builder, values.clone());
    let multiply = &add_multiply(pipl, builder, values.clone());
    let fact = &add_factorial(pipl, builder, values.clone(), greater_than, subtract, multiply);
    let print = &add_print(pipl, builder, values.clone());
    builder.apply(pipl);
    for arg in env::args().skip(1) {
        let x = &values.borrow_mut().name(pipl, N::Usize(usize::from_str_radix(&arg, 10).unwrap()));
        builder.send(fact).names(&[x, print]);
        builder.apply(pipl);
        for _ in 0..999 {
            pipl.step();
        }
    }
}
/*
! fact[x, out] . [greater-than-two, two-or-less]>(x, two, greater-than-two, two-or-less) .
(+
  two-or-less[] . out(x).()
  greater-than-two[] . [t1, t2, t3] .
  (|
    -(x, one, t1) . ()
    t1[x_minus_1] . fact(x_minus_1, t2) . ()
    t2[factorial_x_minus_1] . *(x, factorial_x_minus_1, t3) . ()
    t3[result] . out(result) . ()
  )
))
! >[out][a, b, gt, lte] . call(if a > b then gt else lte -> out) . out() . ()
! -[a, b, out] . call(a - b -> out) . ()
! *[a, b, out] . call(a * b -> out) . ()
! print[arg] . call(print!) . ()
fact(x, print) . ()
*/
