#![allow(dead_code)]
pub use pipl_engine::{Call, Name, Pipl, PiplBuilder, Refs};
use std::cell::RefCell;
use std::hash::Hash;
use std::collections::HashMap;
use std::collections::HashSet;
pub use std::rc::Rc;
#[derive(Debug, Eq, PartialEq)]
pub enum N {
    Bool(bool),
    Usize(usize), Isize(isize),
    U8(u8),       I8(i8),
    U16(u16),     I16(i16),
    U32(u32),     I32(i32),
    U64(u64),     I64(i64),
    U128(u128),   I128(i128),
    Char(char),
    Str(&'static str),
    VecStr(Vec<&'static str>),
    String(String),
}
impl N {
    pub fn bool(v: bool) -> Name<N> { Name::new(N::Bool(v)) }
    pub fn usize(v: usize) -> Name<N> { Name::new(N::Usize(v)) }
    pub fn isize(v: isize) -> Name<N> { Name::new(N::Isize(v)) }
    pub fn u8(v: u8) -> Name<N> { Name::new(N::U8(v)) }
    pub fn i8(v: i8) -> Name<N> { Name::new(N::I8(v)) }
    pub fn u16(v: u16) -> Name<N> { Name::new(N::U16(v)) }
    pub fn i16(v: i16) -> Name<N> { Name::new(N::I16(v)) }
    pub fn u32(v: u32) -> Name<N> { Name::new(N::U32(v)) }
    pub fn i32(v: i32) -> Name<N> { Name::new(N::I32(v)) }
    pub fn u64(v: u64) -> Name<N> { Name::new(N::U64(v)) }
    pub fn i64(v: i64) -> Name<N> { Name::new(N::I64(v)) }
    pub fn u128(v: u128) -> Name<N> { Name::new(N::U128(v)) }
    pub fn i128(v: i128) -> Name<N> { Name::new(N::I128(v)) }
    pub fn char(v: char) -> Name<N> { Name::new(N::Char(v)) }
    pub fn str(v: &'static str) -> Name<N> { Name::new(N::Str(v)) }
    pub fn vec_str(v: Vec<&'static str>) -> Name<N> { Name::new(N::VecStr(v)) }
    pub fn string(v: String) -> Name<N> { Name::new(N::String(v)) }
}
pub fn n(v: &'static str) -> Name<N> { N::str(v) }
#[derive(Debug, Eq, PartialEq)]
pub struct Results(RefCell<HashMap<String, Vec<Refs<N>>>>);
impl Results {
    pub fn new() -> Self {
        Results(RefCell::new(HashMap::new()))
    }
    pub fn log<K: Into<String>>(&self, key: K, refs: Refs<N>) {
        self.0.borrow_mut()
            .entry(key.into())
            .or_insert(Vec::new())
            .push(refs);
    }
    pub fn get(&self, key: &str) -> Vec<Refs<N>> {
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
impl Call<N> for ResultsCall {
    fn call(&self, refs: Refs<N>) -> Refs<N> {
        self.results.log(self.key.clone(), refs.clone());
        refs
    }
}
pub fn log<K: Into<String>>(key: K, results: &Rc<Results>) -> Rc<Call<N>> {
    Rc::new(ResultsCall::new(key, results.clone()))
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
fn assert_eq_refs_list(left: Vec<Refs<N>>, right: Vec<Refs<N>>, key: &String) {
    for (i,(l, r)) in left.iter().zip(right.iter()).enumerate() {
        assert_eq_refs(l, r, key, i);
    }
    assert_eq!(left.len(), right.len(), "results[{:?}].len()", key);
}
fn assert_eq_refs(left: &Refs<N>, right: &Refs<N>, key: &String, i: usize) {
    let keys_left = left.keys();
    let keys_right = right.keys();
    let keys_left = keys_left.iter().collect::<HashSet<_>>();
    let keys_right = keys_right.iter().collect::<HashSet<_>>();
    let (diff_left, diff_right) = diff(&keys_left, &keys_right);
    assert_eq!(diff_left, diff_right, "results[{:?}][{:?}].keys()", key, i);
    for k in keys_left.iter() {
        let left_name = &left.get(k);
        let right_name = &right.get(k);
        assert_eq!(left_name.raw(), right_name.raw(), "results[{:?}][{:?}][{:?}]", key, i, k);
    }
}
pub fn assert_ne_names(left: &Name<N>, right: &Name<N>) {
    assert_ne!(left, right);
}
