use ::name::Name;
pub trait OnRead {
    fn read(&mut self, names: &Vec<Name>);
}
pub trait OnSend {
    fn send(&mut self) -> &Vec<Name>;
}
