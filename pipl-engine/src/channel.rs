use ::name::Name;
use ::refs::Refs;
use std::hash::Hash;
use std::hash::Hasher;
#[derive(Debug)]
pub enum Channel<T> {
    Read(Name<T>),
    Send(Name<T>),
}
impl<T> Channel<T> {
    pub fn read(name: Name<T>) -> Self {
        Channel::Read(name)
    }
    pub fn send(name: Name<T>) -> Self {
        Channel::Send(name)
    }
    pub fn invert(&self) -> Self {
        match self {
            &Channel::Read(ref name) => Self::send(name.clone()),
            &Channel::Send(ref name) => Self::read(name.clone()),
        }
    }
    pub fn translate(&self, refs: &Refs<T>) -> Self {
        let name = refs.get(self.name());
        match self {
            &Channel::Read(_) => Self::read(name),
            &Channel::Send(_) => Self::send(name),
        }
    }
    pub fn name(&self) -> &Name<T> {
        match self {
            &Channel::Read(ref name) => name,
            &Channel::Send(ref name) => name,
        }
    }
}
impl<T> Clone for Channel<T> {
    fn clone(&self) -> Self {
        match self {
            Channel::Send(x) => Channel::Send(x.clone()),
            Channel::Read(x) => Channel::Read(x.clone()),
        }
    }
}
impl<T> Hash for Channel<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (dir, name) = match self {
            Channel::Send(x) => (true, x),
            Channel::Read(x) => (false, x),
        };
        dir.hash(state);
        name.hash(state);
    }
}
impl<T> PartialEq for Channel<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Channel::Send(a), Channel::Send(b)) => a == b,
            (Channel::Read(a), Channel::Read(b)) => a == b,
            _ => false,
        }
    }
}
impl<T> Eq for Channel<T> {}
#[cfg(test)]
mod tests {
    use super::Channel;
    use ::name::Name;
    use ::refs::Refs;
    fn n<T>(name: T) -> Name<T> {
        Name::new(name)
    }
    #[test]
    fn read() {
        let x = n('x');
        assert_eq!(Channel::read(x.clone()), Channel::Read(x));
    }
    #[test]
    fn send() {
        let x = n('x');
        assert_eq!(Channel::send(x.clone()), Channel::Send(x));
    }
    #[test]
    fn invert() {
        let (x, y) = (n('x'), n('y'));
        assert_eq!(Channel::read(x.clone()).invert(), Channel::send(x));
        assert_eq!(Channel::send(y.clone()).invert(), Channel::read(y));
    }
    #[test]
    fn translate() {
        let (x, y) = (n('x'), n('y'));
        let (a, b) = (n('a'), n('b'));
        let read = Channel::read(x.clone());
        let send = Channel::send(y.clone());
        let refs = &mut Refs::new();
        assert_eq!(read.translate(refs), read);
        assert_eq!(send.translate(refs), send);
        refs.set(x, a.clone());
        refs.set(y, b.clone());
        assert_eq!(read.translate(refs), Channel::read(a));
        assert_eq!(send.translate(refs), Channel::send(b));
    }
}
