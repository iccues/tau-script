use crate::object::{object::Object, object_trait::ObjectTrait};

use super::string::String_;
use crate::types::callable::closure::Closure;
use crate::types::compound::tuple::Tuple;
use crate::types::error::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Integer {
    pub value: i32,
}

impl ObjectTrait for Integer {
    fn get_member_fn(this: Object, name: &str) -> Object {
        match name {
            "add" => Closure::new_first(Integer::add, this),
            "to_string" => Closure::new_first(Integer::to_string, this),
            _ => Error::new("get_member not implemented"),
        }
    }
}

impl Integer {
    pub fn new(value: i32) -> Object {
        Self::from_data(Integer { value })
    }


    fn add(input: Object) -> Object {
        match input.get_data_match::<Tuple>().unwrap().elements.as_slice() {
            [a, b] => {
                let a = a.get_data_match::<Integer>().unwrap();
                let b = b.get_data_match::<Integer>().unwrap();
                let result = a.value + b.value;
                Integer::new(result)
            }
            _ => Error::new("Invalid input"),
        }
    }

    fn to_string(input: Object) -> Object {
        match input.get_data_match::<Tuple>().unwrap().elements.as_slice() {
            [a] => {
                let a = a.get_data_match::<Integer>().unwrap();
                String_::new(a.value.to_string())
            }
            _ => Error::new("Invalid input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let int1 = Integer::new(42);
        let int2 = Integer::new(1);

        let sum = int1.get_member("add").call(Tuple::new(vec![int2]));

        assert_eq!(
            sum.get_data::<Integer>().unwrap().value,
            43
        );
    }

    #[test]
    fn test_to_string() {
        let int = Integer::new(42);

        let str_obj = int.get_member("to_string").call(Tuple::new(vec![]));

        assert_eq!(
            str_obj.get_data::<String_>().unwrap().value,
            "42".to_string()
        );
    }
}
