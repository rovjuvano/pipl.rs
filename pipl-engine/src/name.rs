#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Name(pub(crate) usize, pub(crate) usize);
impl Name {
    pub(crate) fn new(name: usize) -> Self {
        Name(name, 0)
    }
    pub fn dup(&self) -> Self {
        Name(self.0, self.1+1)
    }
}
#[cfg(test)]
mod tests {
    use super::Name;
    #[test]
    fn clone_vs_dup() {
        let name = Name::new(1);
        assert_eq!(name, name.clone());
        assert_ne!(name, name.dup());
    }
}
