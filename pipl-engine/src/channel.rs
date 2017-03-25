use ::name::Name;
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
}
