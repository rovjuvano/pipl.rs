#![allow(dead_code)]
pub use pipl_engine::{Call, CallFrame, Name, Pipl, PiplBuilder};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
pub use std::rc::Rc;
#[macro_export]
macro_rules! names {
    (|$pipl:ident| { $($name:ident)* }) => {
        let ( $($name,)* ) = ( $( &$pipl.new_name(N::Str(stringify!($name))), )* );
    };
}
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
pub type Refs = BTreeMap<Name, Name>;
#[derive(Debug, Eq, PartialEq)]
pub struct Results(RefCell<HashMap<String, Vec<Refs>>>);
impl Results {
    pub fn new() -> Self {
        Results(RefCell::new(HashMap::new()))
    }
    pub fn log<K: Into<String>>(&self, key: K, refs: Refs) {
        self.0
            .borrow_mut()
            .entry(key.into())
            .or_insert(Vec::new())
            .push(refs);
    }
    pub fn get(&self, key: &str) -> Vec<Refs> {
        self.0
            .borrow()
            .get(key)
            .or(Some(&Vec::with_capacity(0)))
            .unwrap()
            .clone()
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
    fn call(&self, frame: CallFrame<N>) {
        self.results.log(self.key.clone(), frame.clone_refs());
    }
}
pub fn log<K: Into<String>>(key: K, results: &Rc<Results>) -> Rc<Call<N>> {
    Rc::new(ResultsCall::new(key, results.clone()))
}
fn diff<'a, T: Eq + Hash>(
    left: &'a HashSet<T>,
    right: &'a HashSet<T>,
) -> (HashSet<&'a T>, HashSet<&'a T>) {
    let diff_left = left.difference(&right).collect::<HashSet<_>>();
    let diff_right = right.difference(&left).collect::<HashSet<_>>();
    (diff_left, diff_right)
}
pub fn assert_eq_results(pipl: &Pipl<N>, left: &Rc<Results>, right: &Rc<Results>) {
    let keys_left = left.0.borrow().keys().cloned().collect::<HashSet<_>>();
    let keys_right = right.0.borrow().keys().cloned().collect::<HashSet<_>>();
    let (diff_left, diff_right) = diff(&keys_left, &keys_right);
    assert_eq!(diff_left, diff_right, "results.keys()");
    for ref key in keys_left.iter() {
        assert_eq_refs_list(pipl, left.get(key), right.get(key), key);
    }
}
fn assert_eq_refs_list(pipl: &Pipl<N>, left: Vec<Refs>, right: Vec<Refs>, key: &String) {
    for (i, (l, r)) in left.iter().zip(right.iter()).enumerate() {
        assert_eq_refs(pipl, l, r, key, i);
    }
    assert_eq!(left.len(), right.len(), "results[{:?}].len()", key);
}
fn assert_eq_refs(pipl: &Pipl<N>, left: &Refs, right: &Refs, key: &String, i: usize) {
    let keys_left = left.keys().collect::<HashSet<_>>();
    let keys_right = right.keys().collect::<HashSet<_>>();
    let (diff_left, diff_right) = diff(&keys_left, &keys_right);
    assert_eq!(diff_left, diff_right, "results[{:?}][{:?}].keys()", key, i);
    for k in keys_left.iter() {
        let left_value = pipl.get_value(&left.get(k).unwrap());
        let right_value = pipl.get_value(&right.get(k).unwrap());
        assert_eq!(
            left_value, right_value,
            "results[{:?}][{:?}][{:?}]",
            key, i, k
        );
    }
}
pub fn assert_ne_names(left: &Name, right: &Name) {
    assert_ne!(left, right);
}
