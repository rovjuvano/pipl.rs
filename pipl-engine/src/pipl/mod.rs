pub mod mods;

mod reactions;
use self::reactions::{ReadReaction, SendReaction};

use ::Name;
use ::OnRead;
use ::OnSend;
use ::Mods;
use ::Refs;
use std::rc::Rc;
pub struct Pipl {
    read: Option<ReadReaction>,
    send: Option<SendReaction>,
}
impl Pipl {
    pub fn new() -> Self {
        Pipl {
            read: None,
            send: None,
        }
    }
    pub fn read(&mut self, _name: &Name, read: Rc<OnRead>) {
        self.add_read(ReadReaction::new(read, Refs::new()));
    }
    pub fn send(&mut self, _name: &Name, send: Rc<OnSend>) {
        self.add_send(SendReaction::new(send, Refs::new()));
    }
    fn add_read(&mut self, read: ReadReaction) {
        self.read = Some(read);
    }
    fn add_send(&mut self, send: SendReaction) {
        self.send = Some(send);
    }
    pub fn step(&mut self) {
        if self.read.is_some() && self.send.is_some() {
            let mut mods = Mods::new();
            let reader = self.read.take().unwrap();
            let sender = self.send.take().unwrap();
            let names = sender.send(&mut mods);
            reader.read(&mut mods, names);
            mods.apply(self);
        }
    }
}
