use ::call::{OnRead, OnSend};
use ::name::Name;
pub struct Pipl<'a> {
    read: Option<Box<OnRead + 'a>>,
    send: Option<Box<OnSend + 'a>>,
}
impl<'a> Pipl<'a> {
    pub fn new() -> Self {
        Pipl {
            read: None,
            send: None,
        }
    }
    pub fn read<T>(&mut self, _name: &Name, fun: T) where T: OnRead + 'a {
        self.read = Some(Box::new(fun));
    }
    pub fn send<T>(&mut self, _name: &Name, fun: T) where T: OnSend + 'a {
        self.send = Some(Box::new(fun));
    }
    pub fn step(&mut self) {
        if self.read.is_some() && self.send.is_some() {
            let mut read = self.read.take().unwrap();
            let mut send = self.send.take().unwrap();
            let names = send.send();
            read.read(names);
        }
    }
}
