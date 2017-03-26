use ::process::sequence::Sequence;
use std::rc::Rc;
#[derive(Debug)]
pub struct ChoiceProcess {
    options: Vec<Rc<Sequence>>,
}
impl ChoiceProcess {
    pub fn new(options: Vec<Rc<Sequence>>) -> Self {
        ChoiceProcess { options: options }
    }
    #[inline]
    pub fn options(&self) -> &Vec<Rc<Sequence>> {
        &self.options
    }
}
