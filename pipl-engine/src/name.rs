use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;
pub struct Name<T>(Rc<Rc<T>>);
impl<T> Name<T> {
    pub fn new(name: T) -> Self {
        Name(Rc::new(Rc::new(name)))
    }
    pub fn dup(&self) -> Self {
        Name(Rc::new((*self.0).clone()))
    }
    pub fn raw(&self) -> &T {
        &**self.0
    }
}
impl<T> Clone for Name<T> {
    fn clone(&self) -> Self {
        Name(self.0.clone())
    }
}
impl<T: fmt::Debug> fmt::Debug for Name<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name({:?})", &*self.0)
    }
}
impl<T> Hash for Name<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&*self.0 as * const Rc<T>).hash(state);
    }
}
impl<T> PartialEq for Name<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl<T> Eq for Name<T> {}
#[cfg(test)]
mod tests {
    use super::Name;
    #[test]
    fn clone_vs_dup() {
        let name = Name::new(());
        assert_eq!(name, name.clone());
        assert_ne!(name, name.dup());
    }
}
