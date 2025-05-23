use crate::{object::{object::Object, object_trait::ObjectTrait}, types::{callable::closure::Closure, compound::tuple::Tuple, primitive::string::String_}};

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
        match input.get_data_match::<Tuple>().unwrap().elements.as_slice() {
            [a] => {
                let a = a.get_data_match::<Error>().unwrap();
                String_::new(a.message.clone())
            }
            _ => Error::new("Invalid input"),
        }
    }
}
