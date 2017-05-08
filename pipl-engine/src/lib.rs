mod call;
pub use call::OnRead;
pub use call::OnSend;

mod molecule;
pub use molecule::Molecule;
pub use molecule::ReadMolecule;
pub use molecule::SendMolecule;

mod name;
pub use name::Name;

mod pipl;
pub use pipl::Pipl;
pub use pipl::mods::Mods;

mod refs;
pub use refs::Refs;

// issue #36497: std::ptr::eq unstable
#[inline]
fn ptr_eq<T: ?Sized>(a: *const T, b: *const T) -> bool {
    a == b
}
