extern crate pipl_engine;
use pipl_engine::{Call, Name, Pipl, PiplBuilder, Refs};
use std::env;
use std::fmt;
use std::rc::Rc;
#[derive(Debug)]
enum N {
    String(&'static str),
    Usize(usize),
}
fn n(value: &'static str) -> Name<N> {
    Name::new(N::String(value))
}
fn nn(value: usize) -> Name<N> {
    Name::new(N::Usize(value))
}
impl fmt::Display for N {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      N::String(x) => x.fmt(f),
      N::Usize(x) => x.fmt(f),
    }
  }
}
fn add_factorial(builder: &mut PiplBuilder<N>, greater_than: &Name<N>, subtract: &Name<N>, multiply: &Name<N>) -> Name<N> {
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
                .names(&[x, &nn(2usize), gt2, le2])
                .choice();
    c.read(le2)
        .send(out)
            .names(&[x]);
    let p = c.read(gt2)
        .parallel()
            .restrict(&[t1, t2, t3]);
    p.send(subtract)
        .names(&[x, &nn(1usize), t1]);
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
fn add_print(builder: &mut PiplBuilder<N>) -> Name<N> {
    #[derive(Debug)]
    struct PrintCall(Name<N>);
    impl Call<N> for PrintCall {
        fn call(&self, refs: Refs<N>) -> Refs<N> {
            let s = refs.get(&self.0);
            println!("{}", s.raw());
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
fn add_greater_than(builder: &mut PiplBuilder<N>) -> Name<N> {
    #[derive(Debug)]
    struct GreaterThanCall {
        a: Name<N>,
        b: Name<N>,
        gt: Name<N>,
        lte: Name<N>,
        out: Name<N>,
    }
    impl Call<N> for GreaterThanCall {
        fn call(&self, mut refs: Refs<N>) -> Refs<N> {
            let (tf, result) = match ( refs.get(&self.a).raw(), refs.get(&self.b).raw() ) {
                ( N::Usize(a), N::Usize(b) ) => {
                  let x = if a > b { &self.gt } else { &self.lte };
                  (true, refs.get(x))
                },
                _ => { unreachable!(); },
            };
            if tf { refs.set(self.out.clone(), result) }
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
    a: Name<N>,
    b: Name<N>,
    out: Name<N>,
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
impl Call<N> for BinaryOpCall {
    fn call(&self, mut refs: Refs<N>) -> Refs<N> {
        let (tf, result) = match ( refs.get(&self.a).raw(), refs.get(&self.b).raw() ) {
            ( N::Usize(a), N::Usize(b) ) => (true, (self.f)(*a, *b) ),
            _ => { unreachable!(); },
        };
        if tf { refs.set(self.out.clone(), nn(result)); }
        refs
    }
}
fn add_binary_op<T>(builder: &mut PiplBuilder<N>, label: &str, f: T) -> Name<N>
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
fn add_subtract(builder: &mut PiplBuilder<N>) -> Name<N> {
    add_binary_op(builder, "-", |a, b| { a - b })
}
fn add_multiply(builder: &mut PiplBuilder<N>) -> Name<N> {
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
        builder.send(fact).names(&[&nn(x), print]);
        builder.apply(pipl);
        for _ in 0..999 {
            pipl.step();
        }
    }
}
