use crate::object::{object::Object, object_trait::ObjectTrait};

pub struct Undefined;

impl ObjectTrait for Undefined {}

impl Undefined {
    pub fn new() -> Object {
        Self::from_data(Undefined)
    }
}
