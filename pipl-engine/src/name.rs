use std::fmt;
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Name(Vec<u8>);
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
        Name(name)
    }
}
#[cfg(test)]
mod tests {
    use super::Name;
    #[test]
    fn one() {
        assert_eq!("01", format!("{}", Name(vec!(0x01))));
    }
    #[test]
    fn two() {
        assert_eq!("0123", format!("{}", Name(vec!(0x01, 0x23))));
    }
    #[test]
    fn three() {
        assert_eq!("012345", format!("{}", Name(vec!(0x01, 0x23, 0x45))));
    }
    #[test]
    fn many() {
        assert_eq!("0123456789ABCDEF", format!("{}", Name(vec!(0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef))));
    }
    #[test]
    fn debug() {
        assert_eq!("Name(01EF)", format!("{:?}", Name(vec!(0x01, 0xef))))
    }
}
