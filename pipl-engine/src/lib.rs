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

mod process;
pub use self::process::call_process::CallProcess;
pub use self::process::choice::ChoiceProcess;
pub use self::process::Process;
pub use self::process::parallel::ParallelProcess;
pub use self::process::sequence::Sequence;

mod reaction;

mod refs;
pub use self::refs::Refs;

// issue #36497: std::ptr::eq unstable
#[inline]
pub fn ptr_eq<T: ?Sized>(a: *const T, b: *const T) -> bool {
    a == b
}
