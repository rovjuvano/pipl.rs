use std::any::Any;
use std::fmt;
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name {
    value_id: usize,
    version: usize,
}
impl Name {
    pub(crate) fn new(value_id: usize, version: usize) -> Self {
        Name { value_id, version }
    }
}
impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name({:?}, {:?})", self.value_id, self.version)
    }
}
trait AsAnyDebug: Any + fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}
impl<T: Any + fmt::Debug> AsAnyDebug for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
#[derive(Debug)]
struct Item {
    data: Box<dyn AsAnyDebug>,
    version: usize,
}
impl Item {
    fn new(data: impl AsAnyDebug) -> Self {
        Item {
            data: Box::new(data),
            version: 0,
        }
    }
}
pub struct NameStore {
    values: Vec<Item>,
}
impl NameStore {
    pub(crate) fn new() -> Self {
        NameStore { values: Vec::new() }
    }
    pub fn dup_name(&mut self, name: &Name) -> Name {
        let item = self.values.get_mut(name.value_id).unwrap();
        item.version += 1;
        Name::new(name.value_id, item.version)
    }
    pub fn get_value<T: Any + fmt::Debug>(&self, name: &Name) -> Option<&T> {
        let item = self.values.get(name.value_id).unwrap();
        Any::downcast_ref::<T>((*item.data).as_any())
    }
    pub fn new_name<T: Any + fmt::Debug>(&mut self, data: T) -> Name {
        let name = Name::new(self.values.len(), 0);
        self.values.push(Item::new(data));
        name
    }
}
impl fmt::Debug for NameStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("NameStore")
            .field(&DebugValues(&self.values))
            .finish()
    }
}
struct DebugValues<'a>(&'a Vec<Item>);
impl<'a> fmt::Debug for DebugValues<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(
                self.0
                    .iter()
                    .enumerate()
                    .map(|(i, x)| (DebugKey(i, x.version), &x.data))
                    .collect::<Vec<_>>()
                    .into_iter(),
            ).finish()
    }
}
struct DebugKey(usize, usize);
impl fmt::Debug for DebugKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.0, self.1)
    }
}
