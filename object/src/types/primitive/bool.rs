use crate::object::{object::Object, object_trait::ObjectTrait};

#[derive(Debug, Clone, PartialEq)]
pub struct Bool {
    pub value: bool,
}

impl ObjectTrait for Bool {}

impl Bool {
    pub fn new(value: bool) -> Object {
        Self::from_data(Bool { value })
    }
}
