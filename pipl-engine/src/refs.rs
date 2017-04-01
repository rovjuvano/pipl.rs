use ::name::Name;
use std::collections::HashMap;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Refs {
    refs: HashMap<Name, Name>,
}
impl Refs {
    pub fn new() -> Refs {
        Refs { refs: HashMap::new() }
    }
    pub fn get(&self, key: &Name) -> Name {
        self.refs.get(key).unwrap_or(key).clone()
    }
    pub fn get_names(&self, keys: &[Name]) -> Vec<Name> {
        keys.iter().map(|k| {
            self.get(k)
        }).collect()
    }
    pub fn keys(&self) -> Vec<&Name> {
        self.refs.keys().collect()
    }
    pub fn new_name(&mut self, key: Name) {
        let value = key.dup();
        self.set(key, value);
    }
    pub fn new_names(&mut self, keys: Vec<Name>) {
        for k in keys.into_iter() {
            self.new_name(k);
        }
    }
    pub fn set(&mut self, key: Name, value: Name) {
        self.refs.insert(key, value);
    }
    pub fn set_names(&mut self, keys: Vec<Name>, values: Vec<Name>) {
        for (k, v) in keys.into_iter().zip(values) {
            self.set(k, v);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Name;
    use super::Refs;
    fn n(name: u8) -> Name {
        Name::from(vec!(name))
    }
    #[test]
    fn get_default() {
        let subject = Refs::new();
        let k = n(0x01);
        assert_eq!(subject.get(&k), k.clone());
    }
    #[test]
    fn get_value() {
        let mut subject = Refs::new();
        let (k, v) = (n(0x01), n(0x02));
        subject.refs.insert(k.clone(), v.clone());
        assert_eq!(subject.get(&k), v.clone());
    }
    #[test]
    fn get_names() {
        let mut subject = Refs::new();
        let (k1, v1) = (n(0x01), n(0x02));
        let (k2,)    = (n(0x02),);
        let (k3, v3) = (n(0x03), n(0x03));
        subject.refs.insert(k1.clone(), v1.clone());
        subject.refs.insert(k3.clone(), v3.clone());
        let actual = subject.get_names(&vec![k1, k2.clone(), k3]);
        let expected = vec![v1, k2, v3];
        assert_eq!(actual, expected);
    }
    #[test]
    fn new_name() {
        let mut subject = Refs::new();
        let (k,) = (n(0x01),);
        subject.new_name(k.clone());
        assert_ne!(subject.get(&k), k);
        subject.new_name(k.clone());
        assert_ne!(subject.get(&k), k);
    }
    #[test]
    fn new_names() {
        let mut subject = Refs::new();
        let (k1, k2, k3) = (n(0x01),n(0x02),n(0x03));
        subject.new_names(vec![k1.clone(), k2.clone(), k3.clone()]);
        assert_ne!(subject.get(&k1), k1);
        assert_ne!(subject.get(&k2), k2);
        assert_ne!(subject.get(&k3), k3);
    }
    #[test]
    fn set() {
        let mut subject = Refs::new();
        let (k, v) = (n(0x01), n(0x02));
        subject.set(k.clone(), v.clone());
        assert_eq!(subject.refs.get(&k), Some(&v));
    }
    #[test]
    fn set_names() {
        let mut subject = Refs::new();
        let (k1, v1) = (n(0x0A), n(0x01));
        let (k2, v2) = (n(0x0B), n(0x02));
        let (k3, v3) = (n(0x0C), n(0x03));
        let keys = vec![k1.clone(), k2.clone(), k3.clone()];
        let values = vec![v1.clone(), v2.clone(), v3.clone()];
        subject.set_names(keys, values);
        assert_eq!(subject.refs.get(&k1), Some(&v1));
        assert_eq!(subject.refs.get(&k2), Some(&v2));
        assert_eq!(subject.refs.get(&k3), Some(&v3));
    }
    #[test]
    fn set_names_short_keys() {
        let mut subject = Refs::new();
        let (k1, v1) = (n(0x0A), n(0x01));
        let (k2, v2) = (n(0x0B), n(0x02));
        let keys = vec![k1.clone()];
        let values = vec![v1.clone(), v2.clone()];
        subject.set_names(keys, values);
        assert_eq!(subject.get(&k1), v1.clone());
        assert_eq!(subject.get(&k2), k2.clone());
    }
    #[test]
    fn set_names_short_values() {
        let mut subject = Refs::new();
        let (k1, v1) = (n(0x0A), n(0x01));
        let (k2,) = (n(0x0B),);
        let keys = vec![k1.clone(), k2.clone()];
        let values = vec![v1.clone()];
        subject.set_names(keys, values);
        assert_eq!(subject.get(&k1), v1.clone());
        assert_eq!(subject.get(&k2), k2.clone());
    }
}
