use ::name::Name;
use ::pipl::Pipl;
pub trait OnRead {
    fn read(&mut self, pipl: &mut Pipl, names: &Vec<Name>);
}
pub trait OnSend {
    fn send(&mut self, pipl: &mut Pipl) -> &Vec<Name>;
}
