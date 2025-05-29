use crate::tuple_match;
use crate::{object::{object::Object, object_trait::ObjectTrait}, types::{callable::closure::Closure, error::error::Error}};

use super::string::String_;

#[derive(Debug, Clone, PartialEq)]
pub struct Bool {
    pub value: bool,
}

impl ObjectTrait for Bool {
    fn get_member_fn(this: Object, name: &str) -> Object {
        match name {
            "to_string" => Closure::new_first(Bool::to_string, this),
            _ => Error::new("get_member not implemented"),
        }
    }
}

impl Bool {
    pub fn new(value: bool) -> Object {
        Self::from_data(Bool { value })
    }

    fn to_string(input: Object) -> Object {
        tuple_match!(input, (a: Bool) {
            String_::new(a.value.to_string())
        })
    }
}
