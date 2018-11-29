use std::fmt;
#[derive(Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Name {
    slot_id: usize,
    version: usize,
}
impl Name {
    pub(crate) fn new(slot_id: usize, version: usize) -> Self {
        Name { slot_id, version }
    }
}
impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name({:?}, {:?})", self.slot_id, self.version)
    }
}
pub struct NameStore<T> {
    values: Vec<T>,
    versions: Vec<usize>,
}
impl<T> NameStore<T> {
    pub(crate) fn new() -> Self {
        NameStore {
            values: Vec::new(),
            versions: Vec::new(),
        }
    }
    pub fn dup_name(&mut self, name: &Name) -> Name {
        let version = self.versions.get_mut(name.slot_id).unwrap();
        *version += 1;
        Name::new(name.slot_id, *version)
    }
    pub fn get_value(&self, name: &Name) -> Option<&T> {
        self.values.get(name.slot_id)
    }
    pub fn new_name(&mut self, data: T) -> Name {
        let name = Name::new(self.values.len(), 0);
        self.values.push(data);
        self.versions.push(0);
        name
    }
}
impl<T: fmt::Debug> fmt::Debug for NameStore<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let names = self
            .values
            .iter()
            .zip(self.versions.iter())
            .enumerate()
            .map(|(i, (x, v))| format!("{} [{}]: {:?}", i, v, x))
            .collect::<Vec<String>>();
        if f.alternate() {
            write!(f, "{:#?}", names)
        } else {
            write!(f, "{:?}", names)
        }
    }
}
