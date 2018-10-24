extern crate pipl_engine;
use pipl_engine::{Call, CallFrame, Name, Pipl, PiplBuilder};
use std::env;
use std::fmt;
use std::rc::Rc;
#[derive(Debug)]
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
fn add_factorial(pipl: &mut Pipl<N>, builder: &mut PiplBuilder<N>, greater_than: &Name, subtract: &Name, multiply: &Name) -> Name {
    let fact = pipl.new_name(N::Str("fact"));
    let x = &pipl.new_name(N::Str("x"));
    let out = &pipl.new_name(N::Str("out"));
    let gt2 = &pipl.new_name(N::Str("greater-than-two"));
    let le2 = &pipl.new_name(N::Str("two-or-less"));
    let (t1, t2, t3) = (&pipl.new_name(N::Str("t1")), &pipl.new_name(N::Str("t2")), &pipl.new_name(N::Str("t3")));
    let x_minus_1 = &pipl.new_name(N::Str("x-1"));
    let factorial_x_minus_1 = &pipl.new_name(N::Str("!(x-1)"));
    let result = &pipl.new_name(N::Str("result"));
    let one = &pipl.new_name(N::Usize(1));
    let two = &pipl.new_name(N::Usize(2));
    let c = builder
        .read(&fact)
            .names(&[x, out])
            .repeat()
            .send(greater_than)
                .restrict(&[gt2, le2])
                .names(&[x, two, gt2, le2])
                .choice();
    c.read(le2)
        .send(out)
            .names(&[x]);
    let p = c.read(gt2)
        .parallel()
            .restrict(&[t1, t2, t3]);
    p.send(subtract)
        .names(&[x, one, t1]);
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
fn add_print(pipl: &mut Pipl<N>, builder: &mut PiplBuilder<N>) -> Name {
    #[derive(Debug)]
    struct PrintCall(Name);
    impl Call<N> for PrintCall {
        fn call(&self, frame: CallFrame<N>) {
            println!("{}", frame.get_value(&self.0).unwrap());
        }
    }
    let name = pipl.new_name(N::Str("print"));
    let arg = pipl.new_name(N::Str("arg"));
    builder.read(&name)
        .names(&[&arg])
        .repeat()
        .call(Rc::new(PrintCall(arg)));
    name
}
fn add_greater_than(pipl: &mut Pipl<N>, builder: &mut PiplBuilder<N>) -> Name {
    #[derive(Debug)]
    struct GreaterThanCall {
        a: Name,
        b: Name,
        gt: Name,
        lte: Name,
        out: Name,
    }
    impl Call<N> for GreaterThanCall {
        fn call(&self, mut frame: CallFrame<N>) {
            let maybe = match ( frame.get_value(&self.a), frame.get_value(&self.b) ) {
                ( Some(N::Usize(a)), Some(N::Usize(b)) ) => {
                  let x = if a > b { &self.gt } else { &self.lte };
                  Some(frame.get_name(x))
                },
                _ => None,
            };
            if let Some(result) = maybe {
                frame.set_name(self.out.clone(), result);
            }
        }
    }
    let name = pipl.new_name(N::Str(">"));
    let a = &pipl.new_name(N::Str("a"));
    let b = &pipl.new_name(N::Str("b"));
    let gt = &pipl.new_name(N::Str("gt"));
    let lte = &pipl.new_name(N::Str("lte"));
    let out = &pipl.new_name(N::Str("->"));
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
impl Call<N> for BinaryOpCall {
    fn call(&self, mut frame: CallFrame<N>) {
        let maybe = match ( frame.get_value(&self.a), frame.get_value(&self.b) ) {
            ( Some(N::Usize(a)), Some(N::Usize(b)) ) => Some((self.f)(*a, *b) ),
            _ => None,
        };
        if let Some(result) = maybe {
            let x = frame.new_name(N::Usize(result));
            frame.set_name(self.out.clone(), x);
        }
    }
}
fn add_binary_op<T>(pipl: &mut Pipl<N>, builder: &mut PiplBuilder<N>, label: &'static str, f: T) -> Name
    where T: Fn(usize, usize) -> usize + 'static
{
    let name = pipl.new_name(N::Str(label));
    let a = &pipl.new_name(N::Str("a"));
    let b = &pipl.new_name(N::Str("b"));
    let result = &pipl.new_name(N::Str("="));
    let out = &pipl.new_name(N::Str("->"));
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
fn add_subtract(pipl: &mut Pipl<N>, builder: &mut PiplBuilder<N>) -> Name {
    add_binary_op(pipl, builder, "-", |a, b| { a - b })
}
fn add_multiply(pipl: &mut Pipl<N>, builder: &mut PiplBuilder<N>) -> Name {
    add_binary_op(pipl, builder, "*", |a, b| { a * b })
}
fn main() {
    let pipl = &mut Pipl::new();
    let mut builder = PiplBuilder::new();
    let greater_than = &add_greater_than(pipl, &mut builder);
    let subtract = &add_subtract(pipl, &mut builder);
    let multiply = &add_multiply(pipl, &mut builder);
    let fact = &add_factorial(pipl, &mut builder, greater_than, subtract, multiply);
    let print = &add_print(pipl, &mut builder);
    builder.apply(pipl);
    for arg in env::args().skip(1) {
        let x = &pipl.new_name(
            N::Usize(usize::from_str_radix(&arg, 10).unwrap())
        );
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
