#![allow(dead_code)]
pub use pipl_engine::{Mods, Name, OnRead, OnSend, Pipl, Refs};
use std::cell::RefCell;
use std::fmt;
use std::hash::Hash;
use std::collections::HashMap;
use std::collections::HashSet;
pub use std::rc::Rc;
#[derive(Debug, Eq, PartialEq)]
pub struct Results(RefCell<HashMap<String, Vec<Name>>>);
impl Results {
    pub fn new() -> Rc<Self> {
        Rc::new(Results(RefCell::new(HashMap::new())))
    }
    pub fn log<K: Into<String>>(&self, key: K, value: Name) {
        self.0.borrow_mut()
            .entry(key.into())
            .or_insert(Vec::new())
            .push(value);
    }
    pub fn get(&self, key: &str) -> Vec<Name> {
        self.0.borrow()
            .get(key)
            .or(Some(&Vec::with_capacity(0)))
            .unwrap().clone()
    }
}
pub fn n<T: fmt::Debug +'static>(name: T) -> Name {
    Name::new(name)
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
fn assert_eq_lists(left: Vec<Name>, right: Vec<Name>, key: &str) {
    for (i,(l, r)) in left.iter().zip(right.iter()).enumerate() {
        let message = &format!("results[{:?}][{:?}]", key, i);
        if l.raw().is::<Refs>() {
            assert_eq_refs(l.raw().downcast_ref::<Refs>().unwrap(), r.raw().downcast_ref::<Refs>().unwrap(), message);
        }
        else if l.raw().is::<Vec<Name>>() {
            assert_eq!(l.raw().downcast_ref::<Vec<Name>>(), r.raw().downcast_ref::<Vec<Name>>(), "{}::<Vec<Name>>", message);
        }
        else {
            assert!(false, "unrecognized type for NameValue: {:?}", l);
        }
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
        let message = &format!("{}[{:?}]", message, k);
        if left_name.raw().is::<&str>() {
            assert_eq_name_values::<&str>(left_name, right_name, message);
        }
        else if left_name.raw().is::<char>() {
            assert_eq_name_values::<char>(left_name, right_name, message);
        }
        else {
            assert!(false, "unrecognized type for NameValue: {:?}", left_name);
        }
    }
}
fn assert_eq_name_values<T: fmt::Debug + Eq + PartialEq + 'static>(left: &Name, right: &Name, message: &String) {
    assert_ne!(None, left.raw().downcast_ref::<T>());
    assert_eq!(
        left.raw().downcast_ref::<T>(),
        right.raw().downcast_ref::<T>(),
        "{}",
        message
    );
}
