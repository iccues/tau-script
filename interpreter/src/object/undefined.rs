use crate::object_box::ObjectBox;

use super::{Object, ObjectTrait};

#[derive(Debug)]
pub struct Undefined;

impl Undefined {
    pub fn new() -> Object {
        ObjectBox::new(Undefined {})
    }
}

impl ObjectTrait for Undefined {
    fn to_string_row(&self) -> String {
        "undefined".to_string()
    }
}