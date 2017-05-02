mod name;
pub use name::Name;

mod pipl;
pub use pipl::Pipl;

// issue #36497: std::ptr::eq unstable
#[inline]
fn ptr_eq<T: ?Sized>(a: *const T, b: *const T) -> bool {
    a == b
}
