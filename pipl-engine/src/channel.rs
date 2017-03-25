use ::name::Name;
use ::refs::Refs;
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
    pub fn invert(&self) -> Self {
        match self {
            &Channel::Read(ref name) => Self::send(name.clone()),
            &Channel::Send(ref name) => Self::read(name.clone()),
        }
    }
    pub fn translate(&self, refs: &Refs) -> Self {
        let name = refs.get(self.name());
        match self {
            &Channel::Read(_) => Self::read(name),
            &Channel::Send(_) => Self::send(name),
        }
    }
    pub fn name(&self) -> &Name {
        match self {
            &Channel::Read(ref name) => name,
            &Channel::Send(ref name) => name,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Channel;
    use ::name::Name;
    use ::refs::Refs;
    fn n(name: u8) -> Name {
        Name::from(vec!(name))
    }
    #[test]
    fn read() {
        assert_eq!(Channel::read(n(0)), Channel::Read(n(0)));
    }
    #[test]
    fn send() {
        assert_eq!(Channel::send(n(0)), Channel::Send(n(0)));
    }
    #[test]
    fn invert() {
        assert_eq!(Channel::read(n(0)).invert(), Channel::send(n(0)));
        assert_eq!(Channel::send(n(0)).invert(), Channel::read(n(0)));
    }
    #[test]
    fn translate() {
        let (x, y) = (n(0x78), n(0x79));
        let (a, b) = (n(0x61), n(0x62));
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