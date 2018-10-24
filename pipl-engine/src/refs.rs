use ::name::Name;
use std::collections::HashMap;
#[derive(Debug, Eq, PartialEq)]
pub struct Refs {
    refs: HashMap<Name, Name>,
}
impl Refs {
    pub fn new() -> Self {
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
    pub fn set(&mut self, key: Name, value: Name) {
        self.refs.insert(key, value);
    }
    pub fn set_names(&mut self, keys: Vec<Name>, values: Vec<Name>) {
        for (k, v) in keys.into_iter().zip(values) {
            self.set(k, v);
        }
    }
}
impl Clone for Refs {
    fn clone(&self) -> Self {
        Refs { refs: self.refs.clone() }
    }
}
#[cfg(test)]
mod tests {
    use super::Name;
    use super::Refs;
    fn n(name: usize) -> Name {
        Name::new(name, 0)
    }
    #[test]
    fn get_default() {
        let subject = Refs::new();
        let k = n(1);
        assert_eq!(subject.get(&k), k.clone());
    }
    #[test]
    fn get_value() {
        let mut subject = Refs::new();
        let (k, v) = (n(1), n(2));
        subject.refs.insert(k.clone(), v.clone());
        assert_eq!(subject.get(&k), v.clone());
    }
    #[test]
    fn get_names() {
        let mut subject = Refs::new();
        let (k1, v1) = (n(1), n(2));
        let (k2,)    = (n(2),);
        let (k3, v3) = (n(3), n(3));
        subject.refs.insert(k1.clone(), v1.clone());
        subject.refs.insert(k3.clone(), v3.clone());
        let actual = subject.get_names(&vec![k1, k2.clone(), k3]);
        let expected = vec![v1, k2, v3];
        assert_eq!(actual, expected);
    }
    #[test]
    fn set() {
        let mut subject = Refs::new();
        let (k, v) = (n(1), n(2));
        subject.set(k.clone(), v.clone());
        assert_eq!(subject.refs.get(&k), Some(&v));
    }
    #[test]
    fn set_names() {
        let mut subject = Refs::new();
        let (k1, v1) = (n(11), n(1));
        let (k2, v2) = (n(12), n(2));
        let (k3, v3) = (n(13), n(3));
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
        let (k1, v1) = (n(11), n(1));
        let (k2, v2) = (n(12), n(2));
        let keys = vec![k1.clone()];
        let values = vec![v1.clone(), v2.clone()];
        subject.set_names(keys, values);
        assert_eq!(subject.get(&k1), v1.clone());
        assert_eq!(subject.get(&k2), k2.clone());
    }
    #[test]
    fn set_names_short_values() {
        let mut subject = Refs::new();
        let (k1, v1) = (n(11), n(1));
        let (k2,) = (n(12),);
        let keys = vec![k1.clone(), k2.clone()];
        let values = vec![v1.clone()];
        subject.set_names(keys, values);
        assert_eq!(subject.get(&k1), v1.clone());
        assert_eq!(subject.get(&k2), k2.clone());
    }
}
