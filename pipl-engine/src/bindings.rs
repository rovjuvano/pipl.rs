use crate::name::Name;
use crate::name::NameStore;
use std::collections::BTreeMap;
#[derive(Clone, Debug)]
pub(crate) struct Bindings {
    map: BTreeMap<Name, Name>,
}
impl Bindings {
    pub fn new() -> Self {
        Bindings {
            map: BTreeMap::new(),
        }
    }
    pub fn get_name(&self, key: &Name) -> Name {
        self.map.get(key).unwrap_or(key).clone()
    }
    pub fn get_names(&self, keys: &[Name]) -> Vec<Name> {
        keys.iter().map(|k| self.get_name(k)).collect()
    }
    pub fn new_name<T>(&mut self, names: &mut NameStore<T>, value: T) -> Name {
        let name = names.new_name(value);
        self.map.insert(name.clone(), name.clone());
        name
    }
    pub fn new_names<T>(&mut self, names: &mut NameStore<T>, new_names: &[Name]) {
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
    /// for testing
    pub fn clone_refs(&self) -> BTreeMap<Name, Name> {
        self.map.clone()
    }
}
