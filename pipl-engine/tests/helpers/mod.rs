#![allow(dead_code)]
pub use pipl_engine::{Name, OnRead, OnSend, Pipl};
use std::cell::RefCell;
use std::fmt;
use std::hash::Hash;
use std::collections::HashMap;
use std::collections::HashSet;
pub use std::rc::Rc;
#[derive(Debug, Eq, PartialEq)]
pub struct Results(RefCell<HashMap<String, Vec<String>>>);
impl Results {
    pub fn new() -> Rc<Self> {
        Rc::new(Results(RefCell::new(HashMap::new())))
    }
    pub fn log<K: Into<String>, T: fmt::Debug>(&self, key: K, value: &T) {
        self.0.borrow_mut()
            .entry(key.into())
            .or_insert(Vec::new())
            .push(format!("{:?}", value));
    }
    pub fn get(&self, key: &str) -> Vec<String> {
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
        assert_eq_refs_list(left.get(key), right.get(key), key);
    }
}
fn assert_eq_refs_list(left: Vec<String>, right: Vec<String>, key: &String) {
    for (i,(l, r)) in left.iter().zip(right.iter()).enumerate() {
        assert_eq!(l, r, "results[{:?}][{:?}]", key, i);
    }
    assert_eq!(left.len(), right.len(), "results[{:?}].len()", key);
}
