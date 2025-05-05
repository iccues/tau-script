use crate::object::{object::Object, object_trait::ObjectTrait};

pub struct Error {
    pub message: String,
}

impl ObjectTrait for Error {}

impl Error {
    pub fn new(message: &str) -> Object {
        Self::from_data(Error { message: message.to_string() })
    }
}
