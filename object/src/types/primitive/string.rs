use crate::object::{object::Object, object_trait::ObjectTrait};

pub struct String_ {
    pub value: String,
}

impl ObjectTrait for String_ {}

impl String_ {
    pub fn new(value: String) -> Object {
        Self::from_data(String_ { value })
    }
}
