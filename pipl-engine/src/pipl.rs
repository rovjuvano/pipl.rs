use ::name::Name;
pub struct Pipl<'a> {
    read: Option<Box<Fn(Vec<Name>) + 'a>>,
    send: Option<Box<Fn() -> Vec<Name> + 'a>>,
}
impl<'a> Pipl<'a> {
    pub fn new() -> Self {
        Pipl {
            read: None,
            send: None,
        }
    }
    pub fn read<T>(&mut self, _name: &Name, fun: T) where T: Fn(Vec<Name>) + 'a {
        self.read = Some(Box::new(fun));
    }
    pub fn send<T>(&mut self, _name: &Name, fun: T) where T: Fn() -> Vec<Name> + 'a {
        self.send = Some(Box::new(fun));
    }
    pub fn step(&mut self) {
        if self.read.is_some() && self.send.is_some() {
            let read = self.read.take().unwrap();
            let send = self.send.take().unwrap();
            let names = (*send)();
            (*read)(names);
        }
    }
}
