#![deny(bare_trait_objects)]
mod atom;
mod pipl;

pub use pipl::Pipl;
pub use atom::Atom;

pub fn connect<T>(func: T) where T: Fn(&mut Pipl) {
    pipl::Pipl::connect(func);
}
