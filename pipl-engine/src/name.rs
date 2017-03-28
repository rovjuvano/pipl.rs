use std::fmt;
use std::rc::Rc;
#[derive(Clone, Eq, Hash)]
pub struct Name(Rc<Vec<u8>>);
impl Name {
    pub fn dup(&self) -> Self {
        Name::from((*self.0).clone())
    }
}
impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name({})", self)
    }
}
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.0.iter().map(|x| {
            format!("{:02X}", x)
        }).collect::<Vec<_>>()
        .join("");
        write!(f, "{}", name)
    }
}
impl From<Vec<u8>> for Name {
    fn from(name: Vec<u8>) -> Self {
        Name(Rc::new(name))
    }
}
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        ::ptr_eq(&*self.0, &*other.0)
    }
}
#[cfg(test)]
mod tests {
    use super::Name;
    fn n(name: Vec<u8>) -> Name {
        Name::from(name)
    }
    #[test]
    fn one() {
        assert_eq!("01", format!("{}", n(vec![0x01])));
    }
    #[test]
    fn two() {
        assert_eq!("0123", format!("{}", n(vec![0x01, 0x23])));
    }
    #[test]
    fn three() {
        assert_eq!("012345", format!("{}", n(vec![0x01, 0x23, 0x45])));
    }
    #[test]
    fn many() {
        assert_eq!("0123456789ABCDEF", format!("{}", n(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef])));
    }
    #[test]
    fn debug() {
        assert_eq!("Name(01EF)", format!("{:?}", n(vec![0x01, 0xef])))
    }
    #[test]
    fn clone_vs_dup() {
        let name = n(vec![0x01]);
        assert_eq!(name, name.clone());
        assert_ne!(name, name.dup());
    }
}
