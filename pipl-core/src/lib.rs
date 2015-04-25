mod atom;
mod pipl;

pub use pipl::Pipl;
pub use atom::Atom;

pub fn connect() -> Pipl {
    pipl::connect()
}
