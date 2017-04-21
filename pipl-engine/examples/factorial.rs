extern crate pipl_engine;
use pipl_engine::{Call, Name, Pipl, Prefix, Process, Refs, Sequence};
use std::env;
use std::fmt;
use std::rc::Rc;
fn n<T: fmt::Debug + 'static>(name: T) -> Name {
    Name::new(name)
}
fn add_factorial(pipl: &mut Pipl, greater_than: Name, subtract: Name, multiply: Name) -> Name {
    let fact = n("fact");
    let x = n("x");
    let out = n("out");
    let gt2 = n("greater-than-two");
    let le2 = n("two-or-less");
    let (t1, t2, t3) = (n("t1"), n("t2"), n("t3"));
    let x_minus_1 = n("x-1");
    let factorial_x_minus_1 = n("!(x-1)");
    let result = n("result");
    pipl.add(Sequence::new(
        vec![],
        Prefix::read_many(fact.clone(), vec![x.clone(), out.clone()]),
        Process::new_sequence(vec![gt2.clone(), le2.clone()],
            Prefix::send(greater_than, vec![x.clone(), n(2usize), gt2.clone(), le2.clone()]),
            Process::new_choice(vec![
                Rc::new(Sequence::new(vec![],
                    Prefix::read(le2.clone(), vec![]),
                    Process::new_sequence(vec![],
                        Prefix::send(out.clone(), vec![x.clone()]),
                        Process::Terminal
                    ),
                )),
                Rc::new(Sequence::new(vec![],
                    Prefix::read(gt2.clone(), vec![]),
                    Process::new_names(vec![t1.clone(), t2.clone(), t3.clone()],
                        Process::new_parallel(vec![
                            Rc::new(Sequence::new(vec![],
                                Prefix::send(subtract.clone(), vec![x.clone(), n(1usize), t1.clone()]),
                                Process::Terminal
                            )),
                            Rc::new(Sequence::new(vec![],
                                Prefix::read(t1.clone(), vec![x_minus_1.clone()]),
                                Process::new_sequence(vec![],
                                    Prefix::send(fact.clone(), vec![x_minus_1, t2.clone()]),
                                    Process::Terminal
                                )
                            )),
                            Rc::new(Sequence::new(vec![],
                                Prefix::read(t2.clone(), vec![factorial_x_minus_1.clone()]),
                                Process::new_sequence(vec![],
                                    Prefix::send(multiply.clone(), vec![x.clone(), factorial_x_minus_1.clone(), t3.clone()]),
                                    Process::Terminal
                                )
                            )),
                            Rc::new(Sequence::new(vec![],
                                Prefix::read(t3.clone(), vec![result.clone()]),
                                Process::new_sequence(vec![],
                                    Prefix::send(out.clone(), vec![result.clone()]),
                                    Process::Terminal
                                )
                            ))
                        ])
                    )
                ))
            ])
        )
    ));
    fact
}
fn add_print(pipl: &mut Pipl) -> Name {
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
    pipl.add(Sequence::new(
        vec![],
        Prefix::read_many(name.clone(), vec![arg.clone()]),
        Process::new_call(Rc::new(PrintCall(arg)), Process::Terminal)
    ));
    name
}
fn add_greater_than(pipl: &mut Pipl) -> Name {
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
    let a = n("a");
    let b = n("b");
    let gt = n("gt");
    let lte = n("lte");
    let out = n("->");
    pipl.add(Sequence::new(
        vec![],
        Prefix::read_many(name.clone(), vec![a.clone(), b.clone(), gt.clone(), lte.clone()]),
        Process::new_call(
            Rc::new(GreaterThanCall { a: a, b: b, gt: gt, lte: lte, out: out.clone() }),
            Process::new_sequence(vec![],
                Prefix::send(out, vec![]),
                Process::Terminal
            )
        )
    ));
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
fn add_binary_op<T>(pipl: &mut Pipl, label: &str, f: T) -> Name
    where T: Fn(usize, usize) -> usize + 'static
{
    let name = n("-");
    let a = n("a");
    let b = n("b");
    let result = n("=");
    let out = n("->");
    pipl.add(Sequence::new(
        vec![],
        Prefix::read_many(name.clone(), vec![a.clone(), b.clone(), out.clone()]),
        Process::new_call(
            Rc::new(BinaryOpCall { label: label.to_string(), f: Box::new(f), a: a, b: b, out: result.clone() }),
            Process::new_sequence(vec![],
                Prefix::send(out, vec![result]),
                Process::Terminal
            )
        )
    ));
    name
}
fn add_subtract(pipl: &mut Pipl) -> Name {
    add_binary_op(pipl, "-", |a, b| { a - b })
}
fn add_multiply(pipl: &mut Pipl) -> Name {
    add_binary_op(pipl, "*", |a, b| { a * b })
}
fn main() {
    let pipl = &mut Pipl::new();
    let greater_than = add_greater_than(pipl);
    let subtract = add_subtract(pipl);
    let multiply = add_multiply(pipl);
    let fact = add_factorial(pipl, greater_than, subtract, multiply);
    let print = add_print(pipl);
    for arg in env::args().skip(1) {
        let x = usize::from_str_radix(&arg, 10).unwrap();
        pipl.add(Sequence::new(vec![],
            Prefix::send(fact.clone(), vec![n(x), print.clone()]),
            Process::Terminal
        ));
        for _ in 0..999 {
            pipl.step();
        }
    }
}
