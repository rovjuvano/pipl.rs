use std::any::TypeId;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;
pub trait NameValue: fmt::Debug + 'static {
    fn get_type_id(&self) -> TypeId;
}
impl<T: fmt::Debug + 'static + ?Sized> NameValue for T {
    fn get_type_id(&self) -> TypeId { TypeId::of::<T>() }
}
impl dyn NameValue {
    #[inline]
    pub fn is<T: NameValue>(&self) -> bool {
        TypeId::of::<T>() == self.get_type_id()
    }
    #[inline]
    pub fn downcast_ref<T: NameValue>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe {
                Some(&*(self as *const dyn NameValue as *const T))
            }
        } else {
            None
        }
    }
}
#[derive(Clone)]
pub struct Name(Rc<Rc<dyn NameValue>>);
impl Name {
    pub fn new<T: fmt::Debug + 'static>(name: T) -> Self {
        Name(Rc::new(Rc::new(name)))
    }
    pub fn dup(&self) -> Self {
        Name(Rc::new((*self.0).clone()))
    }
    pub fn raw(&self) -> &dyn NameValue {
        &**self.0
    }
}
impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name({:?})", &*self.0)
    }
}
impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&*self.0 as * const Rc<dyn NameValue>).hash(state);
    }
}
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for Name {}
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
