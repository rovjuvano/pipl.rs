#![allow(dead_code)]
pub use pipl_engine::{Call, Name, Pipl, PiplBuilder, Refs};
use std::cell::RefCell;
use std::fmt;
use std::hash::Hash;
use std::collections::HashMap;
use std::collections::HashSet;
pub use std::rc::Rc;
#[derive(Debug, Eq, PartialEq)]
pub struct Results(RefCell<HashMap<String, Vec<Refs>>>);
impl Results {
    pub fn new() -> Self {
        Results(RefCell::new(HashMap::new()))
    }
    pub fn log<K: Into<String>>(&self, key: K, refs: Refs) {
        self.0.borrow_mut()
            .entry(key.into())
            .or_insert(Vec::new())
            .push(refs);
    }
    pub fn get(&self, key: &str) -> Vec<Refs> {
        self.0.borrow()
            .get(key)
            .or(Some(&Vec::with_capacity(0)))
            .unwrap().clone()
    }
}
#[derive(Debug)]
struct ResultsCall {
    key: String,
    results: Rc<Results>,
}
impl ResultsCall {
    pub fn new<K: Into<String>>(key: K, results: Rc<Results>) -> Self {
        ResultsCall {
            key: key.into(),
            results: results,
        }
    }
}
impl Call for ResultsCall {
    fn call(&self, refs: Refs) -> Refs {
        self.results.log(self.key.clone(), refs.clone());
        refs
    }
}
pub fn log<K: Into<String>>(key: K, results: &Rc<Results>) -> Rc<Call> {
    Rc::new(ResultsCall::new(key, results.clone()))
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
fn assert_eq_refs_list(left: Vec<Refs>, right: Vec<Refs>, key: &String) {
    for (i,(l, r)) in left.iter().zip(right.iter()).enumerate() {
        assert_eq_refs(l, r, key, i);
    }
    assert_eq!(left.len(), right.len(), "results[{:?}].len()", key);
}
fn assert_eq_refs(left: &Refs, right: &Refs, key: &String, i: usize) {
    let keys_left = left.keys();
    let keys_right = right.keys();
    let keys_left = keys_left.iter().collect::<HashSet<_>>();
    let keys_right = keys_right.iter().collect::<HashSet<_>>();
    let (diff_left, diff_right) = diff(&keys_left, &keys_right);
    assert_eq!(diff_left, diff_right, "results[{:?}][{:?}].keys()", key, i);
    for k in keys_left.iter() {
        let left_name = &left.get(k);
        let right_name = &right.get(k);
        let message = &format!("results[{:?}][{:?}][{:?}]", key, i, k);
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
pub fn assert_ne_names(left: &Name, right: &Name) {
    if left.raw().is::<&str>() {
        assert_ne_name_values::<&str>(left, right);
    }
    else {
        assert!(false, "unrecognized type for NameValue: {:?}", left);
    }
    assert_ne!(left, right);
}
fn assert_ne_name_values<T: fmt::Debug + Eq + PartialEq + 'static>(left: &Name, right: &Name) {
    assert_ne!(None, left.raw().downcast_ref::<T>());
    assert_eq!(left.raw().downcast_ref::<T>(), right.raw().downcast_ref::<T>());
}
