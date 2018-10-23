#![allow(dead_code)]
pub use pipl_engine::{Mods, Molecule, Name, OnRead, OnSend, Pipl, ReadMolecule, Refs, SendMolecule};
use std::cell::RefCell;
use std::hash::Hash;
use std::collections::HashMap;
use std::collections::HashSet;
pub use std::rc::Rc;
#[macro_export] macro_rules! names {
    (|$pipl:ident| { $($name:ident)* }) => {
        let ( $($name,)* ) = ( $( &$pipl.new_name(N::Str(stringify!($name))), )* );
    };
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum N {
    Str(&'static str),
    Refs(Refs),
    Vec(Vec<Name>),
}
impl<'a> From<&'a N> for String {
    fn from(name: &'a N) -> String {
        match name {
            N::Str(x) => String::from(*x),
            N::Refs(x) => format!("{:?}", x),
            _ => unimplemented!(),
            // N::Vec(x) => format!("{}", x.iter().map(|x| String::from(x.raw()) ).collect::<String>(),)
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub struct Results(RefCell<HashMap<String, Vec<N>>>);
impl Results {
    pub fn new() -> Rc<Self> {
        Rc::new(Results(RefCell::new(HashMap::new())))
    }
    pub fn log<K: Into<String>>(&self, key: K, value: N) {
        self.0.borrow_mut()
            .entry(key.into())
            .or_insert(Vec::new())
            .push(value);
    }
    pub fn get(&self, key: &str) -> Vec<N> {
        self.0.borrow()
            .get(key)
            .or(Some(&Vec::with_capacity(0)))
            .unwrap().clone()
    }
}
pub fn unslice<T: Clone>(slice: &[&T]) -> Vec<T> {
    slice.iter().map(|&x| x.clone()).collect()
}
pub fn read(name: &Name, read: Rc<OnRead<N>>) -> Molecule<N> {
    Molecule::from(ReadMolecule::new(name.clone(), read))
}
pub fn send(name: &Name, send: Rc<OnSend<N>>) -> Molecule<N> {
    Molecule::from(SendMolecule::new(name.clone(), send))
}
fn diff<'a, T: Eq + Hash>(left: &'a HashSet<T>, right: &'a HashSet<T>) -> (HashSet<&'a T>, HashSet<&'a T>) {
    let diff_left = left.difference(&right).collect::<HashSet<_>>();
    let diff_right = right.difference(&left).collect::<HashSet<_>>();
    (diff_left, diff_right)
}
pub fn assert_eq_results(left: &Rc<Results>, right: &Rc<Results>) {
    let keys_left = left.0.borrow().keys().cloned().collect::<HashSet<_>>();
    let keys_right = right.0.borrow().keys().cloned().collect::<HashSet<_>>();
    let (diff_left, diff_right) = diff(&keys_left, &keys_right);
    assert_eq!(diff_left, diff_right, "results.keys()");
    for ref key in keys_left.iter() {
        assert_eq_lists(left.get(key), right.get(key), key);
    }
}
fn assert_eq_lists(left: Vec<N>, right: Vec<N>, key: &str) {
    for (i,(l, r)) in left.iter().zip(right.iter()).enumerate() {
        assert_eq!(l, r, "results[{:?}][{:?}]", key, i);
    }
    assert_eq!(left.len(), right.len(), "results[{:?}].len()", key);
}
fn assert_eq_refs(left: &Refs, right: &Refs, message: &str) {
    let keys_left = left.keys();
    let keys_right = right.keys();
    let keys_left = keys_left.iter().collect::<HashSet<_>>();
    let keys_right = keys_right.iter().collect::<HashSet<_>>();
    let (diff_left, diff_right) = diff(&keys_left, &keys_right);
    assert_eq!(diff_left, diff_right, "{}.keys()", message);
    for k in keys_left.iter() {
        let left_name = &left.get(k);
        let right_name = &right.get(k);
        assert_eq!(left_name, right_name, "{}[{:?}]", message, k);
    }
}
