use ::process::sequence::Sequence;
use std::rc::Rc;
#[derive(Debug)]
pub struct ParallelProcess {
    sequences: Vec<Rc<Sequence>>,
}
impl ParallelProcess {
    pub fn new(sequences: Vec<Rc<Sequence>>) -> Self {
        ParallelProcess { sequences: sequences }
    }
    #[inline]
    pub fn sequences(&self) -> &Vec<Rc<Sequence>> {
        &self.sequences
    }
}
