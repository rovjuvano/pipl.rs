use helpers::*;
use std::cell::RefCell;
#[derive(Debug)]
struct Choice {
    options: Rc<RefCell<Vec<Molecule>>>,
    next: Molecule,
}
impl OnRead for Choice {
    fn read(&self, mods: &mut Mods, _read: ReadMolecule, refs: Refs, names: Vec<Name>) {
        for molecule in self.options.borrow().iter() {
            mods.remove(molecule.clone(), refs.clone());
        }
        if let Molecule::Read(ref next) = self.next {
            next.clone().read(mods, refs, names);
        }
    }
}
impl OnSend for Choice {
    fn send(&self, mods: &mut Mods, _send: SendMolecule, refs: Refs) -> Vec<Name> {
        for molecule in self.options.borrow().iter() {
            mods.remove(molecule.clone(), refs.clone());
        }
        if let Molecule::Send(ref next) = self.next {
            next.clone().send(mods, refs)
        }
        else {
            Vec::new()
        }
    }
}
#[derive(Debug)]
struct Read {
    results: Rc<Results>,
    names: Vec<Name>,
    next: Vec<Molecule>,
}
impl Read {
    fn new(results: &Rc<Results>, names: &[&Name]) -> Rc<Self> {
        Self::new_then(results, names, Vec::new())
    }
    fn new_then(results: &Rc<Results>, names: &[&Name], next: Vec<Molecule>) -> Rc<Self> {
        Rc::new(Read {
            results: results.clone(),
            names: unslice(names),
            next: next,
        })
    }
}
impl OnRead for Read {
    fn read(&self, mods: &mut Mods, read: ReadMolecule, mut refs: Refs, names: Vec<Name>) {
        refs.set_names(self.names.clone(), names.clone());
        self.results.log(format!("{}", read.name().raw().downcast_ref::<&str>().unwrap()), Name::new(refs.clone()));
        let options = Rc::new(RefCell::new(Vec::new()));
        for molecule in self.next.iter() {
            let choice = Rc::new(Choice { options: options.clone(), next: molecule.clone() });
            let choice = match *molecule {
                Molecule::Read(ref read) => Molecule::from(ReadMolecule::new(read.name().clone(), choice)),
                Molecule::Send(ref send) => Molecule::from(SendMolecule::new(send.name().clone(), choice)),
            };
            options.borrow_mut().push(choice.clone());
            mods.add(choice, refs.clone());
        }
    }
}
#[derive(Debug)]
struct Send {
    names: Vec<Name>,
    next: Option<Molecule>,
}
impl Send {
    fn new(names: &[&Name]) -> Rc<Self> {
        Rc::new(Send {
            names: names.iter().map(|&x| x.clone()).collect(),
            next: None,
        })
    }
    fn new_then(names: &[&Name], next: Molecule) -> Rc<Self> {
        Rc::new(Send {
            names: unslice(names),
            next: Some(next),
        })
    }
}
impl OnSend for Send {
    fn send(&self, mods: &mut Mods, _send: SendMolecule, refs: Refs) -> Vec<Name> {
        let names = refs.get_names(&self.names);
        if let Some(ref next) = self.next {
            mods.add(next.clone(), refs);
        }
        names
    }
}
#[test]
fn terminate_choice() {
    // w[x].(+ x[y].() y[z].y[z].() ) w(a).a(b).y(c).() b(d).()
    let (w, x, y, z) = (&n("w"), &n("x"), &n("y"), &n("z"));
    let (a, b, c, d) = (&n("a"), &n("b"), &n("c"), &n("d"));
    let actual = &Results::new();
    let wx = read(w, Read::new_then(actual, &[x], vec![
        read(x, Read::new(actual, &[y])),
        read(y, Read::new_then(actual, &[z], vec![
            read(y, Read::new(actual, &[z]))
        ])),
    ]));
    let wa = send(w, Send::new_then(&[a],
        send(a, Send::new_then(&[b],
            send(y, Send::new(&[c]))
        ))
    ));
    let bd = send(b, Send::new(&[d]));
    let mut pipl = Pipl::new();
    pipl.add(wx);
    pipl.add(wa);
    pipl.add(bd);
    let expected = &Results::new();
    let refs_wx = &mut Refs::new();
    refs_wx.set(x.clone(), a.clone());
    expected.log("w", Name::new(refs_wx.clone()));
    refs_wx.set(y.clone(), b.clone());
    expected.log("x", Name::new(refs_wx.clone()));
    pipl.step();
    pipl.step();
    pipl.step();
    pipl.step();
    assert_eq_results(actual, expected);
}
