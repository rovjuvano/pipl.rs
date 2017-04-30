mod builder;
pub use self::builder::{ChoiceBuilder, ParallelBuilder, PiplBuilder, PrefixBuilder};

mod call;
pub use self::call::Call;

mod channel;

mod pipl;
pub use self::pipl::Pipl;

mod name;
pub use self::name::Name;

mod prefix;

mod reaction;

mod refs;
pub use self::refs::Refs;

// issue #36497: std::ptr::eq unstable
#[inline]
fn ptr_eq<T: ?Sized>(a: *const T, b: *const T) -> bool {
    a == b
}
