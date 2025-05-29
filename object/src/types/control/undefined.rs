use crate::{
    object::{object::Object, object_trait::ObjectTrait},
    tuple_match,
    types::{callable::closure::Closure, error::error::Error, primitive::string::String_},
};

pub struct Undefined;

impl ObjectTrait for Undefined {
    fn get_member_fn(this: Object, name: &str) -> Object {
        match name {
            "to_string" => Closure::new_first(Undefined::to_string, this),
            _ => Error::new("get_member not implemented"),
        }
    }
}

impl Undefined {
    pub fn new() -> Object {
        Self::from_data(Undefined)
    }

    fn to_string(input: Object) -> Object {
        tuple_match!(input, (_a: Undefined) {
            String_::new("undefined".to_string())
        })
    }
}
