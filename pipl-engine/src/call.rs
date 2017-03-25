use ::refs::Refs;
use std::fmt::Debug;
pub trait Call: Debug {
    fn call(&self, refs: Refs) -> Refs;
}
