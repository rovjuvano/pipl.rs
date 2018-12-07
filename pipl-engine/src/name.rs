use std::any::Any;
use std::cmp;
use std::fmt;
use std::hash;
#[derive(Clone, Eq)]
pub struct Name {
    name_id: usize,
    value_id: usize,
}
impl Name {
    pub(crate) fn new(name_id: usize, value_id: usize) -> Self {
        Name { name_id, value_id }
    }
}
impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name({:?}, {:?})", self.name_id, self.value_id)
    }
}
impl hash::Hash for Name {
    fn hash<H: hash::Hasher>(&self, h: &mut H) {
        self.name_id.hash(h)
    }
}
impl Ord for Name {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.name_id.cmp(&other.name_id)
    }
}
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.name_id == other.name_id
    }
}
impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
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
}
impl Item {
    fn new(data: impl AsAnyDebug) -> Self {
        Item {
            data: Box::new(data),
        }
    }
}
pub struct NameStore {
    next_id: usize,
    values: Vec<Item>,
}
impl NameStore {
    pub(crate) fn new() -> Self {
        NameStore {
            next_id: 0,
            values: Vec::new(),
        }
    }
    pub fn dup_name(&mut self, name: &Name) -> Name {
        self.next_name(name.value_id)
    }
    pub fn get_value<T: Any + fmt::Debug>(&self, name: &Name) -> Option<&T> {
        let item = self.values.get(name.value_id).unwrap();
        Any::downcast_ref::<T>((*item.data).as_any())
    }
    pub fn new_name<T: Any + fmt::Debug>(&mut self, data: T) -> Name {
        let value_id = self.values.len();
        self.values.push(Item::new(data));
        self.next_name(value_id)
    }
    fn next_name(&mut self, value_id: usize) -> Name {
        let name = Name::new(self.next_id, value_id);
        self.next_id += 1;
        name
    }
}
impl fmt::Debug for NameStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NameStore")
            .field("next_id", &self.next_id)
            .field("values", &DebugValues(&self.values))
            .finish()
    }
}
struct DebugValues<'a>(&'a Vec<Item>);
impl<'a> fmt::Debug for DebugValues<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let values = self
            .0
            .iter()
            .enumerate()
            .map(|(i, x)| (i, &x.data))
            .collect::<Vec<_>>();
        f.debug_map().entries(values.into_iter()).finish()
    }
}
