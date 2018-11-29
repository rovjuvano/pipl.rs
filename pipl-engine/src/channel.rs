use crate::name::Name;
use std::hash::Hash;
use std::hash::Hasher;
#[derive(Debug)]
pub enum Channel {
    Read(Name),
    Send(Name),
}
impl Channel {
    pub fn read(name: Name) -> Self {
        Channel::Read(name)
    }
    pub fn send(name: Name) -> Self {
        Channel::Send(name)
    }
    pub fn clone_with(&self, name: Name) -> Self {
        match self {
            &Channel::Read(_) => Self::read(name),
            &Channel::Send(_) => Self::send(name),
        }
    }
    pub fn invert(&self) -> Self {
        match self {
            &Channel::Read(ref name) => Self::send(name.clone()),
            &Channel::Send(ref name) => Self::read(name.clone()),
        }
    }
    pub fn name(&self) -> &Name {
        match self {
            &Channel::Read(ref name) => name,
            &Channel::Send(ref name) => name,
        }
    }
}
impl Clone for Channel {
    fn clone(&self) -> Self {
        match self {
            Channel::Send(x) => Channel::Send(x.clone()),
            Channel::Read(x) => Channel::Read(x.clone()),
        }
    }
}
impl Hash for Channel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (dir, name) = match self {
            Channel::Send(x) => (true, x),
            Channel::Read(x) => (false, x),
        };
        dir.hash(state);
        name.hash(state);
    }
}
impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Channel::Send(a), Channel::Send(b)) => a == b,
            (Channel::Read(a), Channel::Read(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for Channel {}
#[cfg(test)]
mod tests {
    use super::Channel;
    use crate::name::Name;
    fn n(name: char) -> Name {
        Name::new((name.to_digit(36).unwrap() as u8).into(), 0)
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
    fn clone_with() {
        let (x, y) = (n('x'), n('y'));
        let (a, b) = (n('a'), n('b'));
        let read = Channel::read(x);
        let send = Channel::send(y);
        assert_eq!(read.clone_with(a.clone()), Channel::read(a));
        assert_eq!(send.clone_with(b.clone()), Channel::send(b));
    }
}
