mod index;
mod store;

use self::index::ContextIndex;
pub(in pipl) use self::store::ContextStore;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ContextId(usize);
