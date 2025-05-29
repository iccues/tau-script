use crate::{object::{object::Object, object_trait::ObjectTrait}, tuple_match, types::{callable::closure::Closure, primitive::string::String_}};

pub struct Error {
    pub message: String,
}

impl ObjectTrait for Error {
    fn get_member_fn(this: Object, name: &str) -> Object {
        match name {
            "to_string" => Closure::new_first(Error::to_string, this),
            _ => Error::new("get_member not implemented"),
        }
    }
}

impl Error {
    pub fn new(message: &str) -> Object {
        Self::from_data(Error { message: message.to_string() })
    }

    fn to_string(input: Object) -> Object {
        tuple_match!(input, (a: Error) {
            String_::new(a.message.clone())
        })
    }
}
