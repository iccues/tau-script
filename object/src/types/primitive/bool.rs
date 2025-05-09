use crate::{object::{object::Object, object_trait::ObjectTrait}, types::{callable::closure::Closure, compound::tuple::Tuple, error::error::Error}};

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
        match input.get_data_match::<Tuple>().unwrap().elements.as_slice() {
            [a] => {
                let a = a.get_data_match::<Bool>().unwrap();
                String_::new(a.value.to_string())
            }
            _ => Error::new("Invalid input"),
        }
    }
}
