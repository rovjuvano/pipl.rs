use crate::name::Name;
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
