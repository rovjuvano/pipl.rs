extern crate pipl_engine;
use pipl_engine::{Call, Name, Pipl, PiplBuilder, Refs};
use std::env;
use std::fmt;
use std::rc::Rc;
fn n<T: fmt::Debug + 'static>(name: T) -> Name {
    Name::new(name)
}
fn add_factorial(builder: &mut PiplBuilder, greater_than: &Name, subtract: &Name, multiply: &Name) -> Name {
    let fact = n("fact");
    let x = &n("x");
    let out = &n("out");
    let gt2 = &n("greater-than-two");
    let le2 = &n("two-or-less");
    let (t1, t2, t3) = (&n("t1"), &n("t2"), &n("t3"));
    let x_minus_1 = &n("x-1");
    let factorial_x_minus_1 = &n("!(x-1)");
    let result = &n("result");
    let c = builder
        .read(&fact)
            .names(&[x, out])
            .repeat()
            .send(greater_than)
                .restrict(&[gt2, le2])
                .names(&[x, &n(2usize), gt2, le2])
                .choice();
    c.read(le2)
        .send(out)
            .names(&[x]);
    let p = c.read(gt2)
        .parallel()
            .restrict(&[t1, t2, t3]);
    p.send(subtract)
        .names(&[x, &n(1usize), t1]);
    p.read(t1)
        .names(&[x_minus_1])
        .send(&fact)
            .names(&[x_minus_1, t2]);
    p.read(t2)
        .names(&[factorial_x_minus_1])
        .send(multiply)
            .names(&[x, factorial_x_minus_1, t3]);
    p.read(t3)
        .names(&[result])
        .send(out)
            .names(&[result]);
    fact
}
fn add_print(builder: &mut PiplBuilder) -> Name {
    #[derive(Debug)]
    struct PrintCall(Name);
    impl Call for PrintCall {
        fn call(&self, refs: Refs) -> Refs {
            let s = refs.get(&self.0);
            if s.raw().is::<usize>() {
                println!("{}", s.raw().downcast_ref::<usize>().unwrap());
            } else {
                println!("Unknown value type: {:?}", s);
            }
            refs
        }
    }
    let name = n("print");
    let arg = n("arg");
    builder.read(&name)
        .names(&[&arg])
        .repeat()
        .call(Rc::new(PrintCall(arg)));
    name
}
fn add_greater_than(builder: &mut PiplBuilder) -> Name {
    #[derive(Debug)]
    struct GreaterThanCall {
        a: Name,
        b: Name,
        gt: Name,
        lte: Name,
        out: Name,
    }
    impl Call for GreaterThanCall {
        fn call(&self, mut refs: Refs) -> Refs {
            if refs.get(&self.a).raw().is::<usize>() && refs.get(&self.b).raw().is::<usize>() {
                let a = *refs.get(&self.a).raw().downcast_ref::<usize>().unwrap();
                let b = *refs.get(&self.b).raw().downcast_ref::<usize>().unwrap();
                let result = refs.get(if a > b { &self.gt } else { &self.lte });
                refs.set(self.out.clone(), result);
            }
            refs
        }
    }
    let name = n(">");
    let a = &n("a");
    let b = &n("b");
    let gt = &n("gt");
    let lte = &n("lte");
    let out = &n("->");
    let gt_call = GreaterThanCall { a: a.clone(), b: b.clone(), gt: gt.clone(), lte: lte.clone(), out: out.clone() };
    builder.read(&name)
        .names(&[a, b, gt, lte])
        .repeat()
        .call(Rc::new(gt_call))
        .send(out);
    name
}
struct BinaryOpCall {
    label: String,
    f: Box<Fn(usize, usize) -> usize>,
    a: Name,
    b: Name,
    out: Name,
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
    fn call(&self, mut refs: Refs) -> Refs {
        if refs.get(&self.a).raw().is::<usize>() && refs.get(&self.b).raw().is::<usize>() {
            let a = *refs.get(&self.a).raw().downcast_ref::<usize>().unwrap();
            let b = *refs.get(&self.b).raw().downcast_ref::<usize>().unwrap();
            let result = (self.f)(a, b);
            refs.set(self.out.clone(), Name::new(result));
        }
        refs
    }
}
fn add_binary_op<T>(builder: &mut PiplBuilder, label: &str, f: T) -> Name
    where T: Fn(usize, usize) -> usize + 'static
{
    let name = n("-");
    let a = &n("a");
    let b = &n("b");
    let result = &n("=");
    let out = &n("->");
    let op = BinaryOpCall { label: label.to_string(), f: Box::new(f), a: a.clone(), b: b.clone(), out: result.clone() };
    builder
        .read(&name)
            .names(&[a, b, out])
            .repeat()
            .call(Rc::new(op))
        .send(out)
           .names(&[&result]);
    name
}
fn add_subtract(builder: &mut PiplBuilder) -> Name {
    add_binary_op(builder, "-", |a, b| { a - b })
}
fn add_multiply(builder: &mut PiplBuilder) -> Name {
    add_binary_op(builder, "*", |a, b| { a * b })
}
fn main() {
    let pipl = &mut Pipl::new();
    let mut builder = PiplBuilder::new();
    let greater_than = &add_greater_than(&mut builder);
    let subtract = &add_subtract(&mut builder);
    let multiply = &add_multiply(&mut builder);
    let fact = &add_factorial(&mut builder, greater_than, subtract, multiply);
    let print = &add_print(&mut builder);
    builder.apply(pipl);
    for arg in env::args().skip(1) {
        let x = usize::from_str_radix(&arg, 10).unwrap();
        builder.send(fact).names(&[&n(x), print]);
        builder.apply(pipl);
        for _ in 0..999 {
            pipl.step();
        }
    }
}