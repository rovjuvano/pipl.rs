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
        let k = 0x01;
        assert_eq!(subject.get(&n(k)), n(k));
    }
    #[test]
    fn get_value() {
        let mut subject = Refs::new();
        let (k, v) = (0x01, 0x02);
        subject.refs.insert(n(k), n(v));
        assert_eq!(subject.get(&n(k)), n(v));
    }
    #[test]
    fn get_names() {
        let mut subject = Refs::new();
        let (k1, v1) = (0x01, 0x02);
        let (k2, v2) = (0x02, 0x02);
        let (k3, v3) = (0x03, 0x03);
        subject.refs.insert(n(k1), n(v1));
        subject.refs.insert(n(k3), n(v3));
        let actual = subject.get_names(&vec![n(k1), n(k2), n(k3)]);
        let expected = vec![n(v1), n(v2), n(v3)];
        assert_eq!(actual, expected);
    }
    #[test]
    fn set() {
        let mut subject = Refs::new();
        let (k, v) = (0x01, 0x02);
        subject.set(n(k), n(v));
        assert_eq!(subject.refs.get(&n(k)), Some(&n(v)));
    }
    #[test]
    fn set_names() {
        let mut subject = Refs::new();
        let (k1, v1) = (0x0A, 0x01);
        let (k2, v2) = (0x0B, 0x02);
        let (k3, v3) = (0x0C, 0x03);
        let keys = vec![n(k1), n(k2), n(k3)];
        let values = vec![n(v1), n(v2), n(v3)];
        subject.set_names(keys, values);
        assert_eq!(subject.refs.get(&n(k1)), Some(&n(v1)));
        assert_eq!(subject.refs.get(&n(k2)), Some(&n(v2)));
        assert_eq!(subject.refs.get(&n(k3)), Some(&n(v3)));
    }
    #[test]
    fn set_names_short_keys() {
        let mut subject = Refs::new();
        let (k1, v1) = (0x0A, 0x01);
        let (k2, v2) = (0x0B, 0x02);
        let keys = vec![n(k1)];
        let values = vec![n(v1), n(v2)];
        subject.set_names(keys, values);
        assert_eq!(subject.get(&n(k1)), n(v1));
        assert_eq!(subject.get(&n(k2)), n(k2));
    }
    #[test]
    fn set_names_short_values() {
        let mut subject = Refs::new();
        let (k1, v1) = (0x0A, 0x01);
        let (k2,) = (0x0B,);
        let keys = vec![n(k1), n(k2)];
        let values = vec![n(v1)];
        subject.set_names(keys, values);
        assert_eq!(subject.get(&n(k1)), n(v1));
        assert_eq!(subject.get(&n(k2)), n(k2));
    }
}
