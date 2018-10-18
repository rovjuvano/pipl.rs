use ::refs::Refs;
use std::fmt::Debug;
pub trait Call<T>: Debug {
    fn call(&self, refs: Refs<T>) -> Refs<T>;
}
