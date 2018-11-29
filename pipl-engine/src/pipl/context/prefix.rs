use crate::name::Name;
use crate::name::NameStore;
use crate::pipl::context::ChoiceContext;
use crate::prefix::Prefix;
use std::collections::BTreeMap;
use std::rc::Rc;
#[derive(Debug)]
pub(crate) struct PrefixContext<T> {
    pub map: BTreeMap<Name, Name>,
    pub prefix: Rc<Prefix<T>>,
}
impl<T> PrefixContext<T> {
    pub fn choice(self, prefixes: Vec<Rc<Prefix<T>>>) -> ChoiceContext<T> {
        ChoiceContext {
            map: Rc::new(self.map),
            prefixes: Rc::new(prefixes),
        }
    }
    pub fn clone_with(&self, prefix: Rc<Prefix<T>>) -> Self {
        PrefixContext {
            map: self.map.clone(),
            prefix,
        }
    }
    pub fn get_name(&self, key: &Name) -> Name {
        self.map.get(key).unwrap_or(key).clone()
    }
    pub fn get_names(&self, keys: &[Name]) -> Vec<Name> {
        keys.iter().map(|k| self.get_name(k)).collect()
    }
    pub fn new_name(&mut self, names: &mut NameStore<T>, value: T) -> Name {
        let name = names.new_name(value);
        self.map.insert(name.clone(), name.clone());
        name
    }
    pub fn new_names(&mut self, names: &mut NameStore<T>, new_names: &[Name]) {
        for name in new_names.iter() {
            self.map.insert(name.clone(), names.dup_name(name));
        }
    }
    pub fn set_name(&mut self, key: Name, value: Name) {
        self.map.insert(key, value);
    }
    pub fn set_names(&mut self, keys: &[Name], values: Vec<Name>) {
        for (k, v) in keys.iter().zip(values) {
            self.set_name(k.clone(), v);
        }
    }
}
impl<T> Clone for PrefixContext<T> {
    fn clone(&self) -> Self {
        PrefixContext {
            map: self.map.clone(),
            prefix: self.prefix.clone(),
        }
    }
}
