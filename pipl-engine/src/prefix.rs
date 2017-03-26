use ::channel::Channel;
use ::name::Name;
use std::fmt;
#[derive(Debug)]
pub struct Prefix {
    channel: Channel,
    names: Vec<Name>,
    repeating: bool,
}
impl Prefix {
    pub fn read(channel: Name, names: Vec<Name>, repeating: bool) -> Prefix {
        Prefix {
            channel: Channel::read(channel),
            names: names,
            repeating: repeating,
        }
    }
    pub fn send(channel: Name, names: Vec<Name>, repeating: bool) -> Prefix {
        Prefix {
            channel: Channel::send(channel),
            names: names,
            repeating: repeating,
        }
    }
    #[inline]
    pub fn channel(&self) -> &Channel {
        &self.channel
    }
    #[inline]
    pub fn is_repeating(&self) -> bool {
        self.repeating
    }
    #[inline]
    pub fn names(&self) -> &Vec<Name> {
        &self.names
    }
}
impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bang = if self.repeating { "!" } else { "" };
        let (open, channel, close) = match self.channel {
            Channel::Read(ref channel) => ('[', channel, ']'),
            Channel::Send(ref channel) => ('(', channel, ')'),
        };
        let names = self.names.iter().map(|x| {
            format!("{}", x)
        }).collect::<Vec<_>>()
        .join(", ");
        write!(f, "{}{}{}{}{}", bang, channel, open, names, close)
    }
}
#[cfg(test)]
mod tests {
    use super::Name;
    use super::Prefix;
    fn n(name: u8) -> Name {
        Name::from(vec!(name))
    }
    #[test]
    fn read_one() {
        let subject = Prefix::read(n(0), vec![n(1)], false);
        assert_eq!("00[01]", format!("{}", subject));
    }
    #[test]
    fn send_one() {
        let subject = Prefix::send(n(0), vec![n(1)], false);
        assert_eq!("00(01)", format!("{}", subject));
    }
    #[test]
    fn read_many() {
        let subject = Prefix::read(n(0), vec![n(1), n(2), n(3)], false);
        assert_eq!("00[01, 02, 03]", format!("{}", subject));
    }
    #[test]
    fn send_many() {
        let subject = Prefix::send(n(0), vec![n(1), n(2), n(3)], false);
        assert_eq!("00(01, 02, 03)", format!("{}", subject));
    }
    #[test]
    fn read_repeating() {
        let subject = Prefix::read(n(0), vec![n(1)], true);
        assert_eq!("!00[01]", format!("{}", subject));
    }
    #[test]
    fn send_repeating() {
        let subject = Prefix::send(n(0), vec![n(1)], true);
        assert_eq!("!00(01)", format!("{}", subject));
    }
}
