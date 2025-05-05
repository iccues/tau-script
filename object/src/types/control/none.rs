use crate::object::{object::Object, object_trait::ObjectTrait};

pub struct None;

impl ObjectTrait for None {}

impl None {
    pub fn new() -> Object {
        Self::from_data(None)
    }
}
