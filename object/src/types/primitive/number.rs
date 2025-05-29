use crate::object::{object::Object, object_trait::ObjectTrait};
use crate::tuple_match;

use super::bool::Bool;
use super::string::String_;
use crate::types::callable::closure::Closure;
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
            "eq" => Closure::new_first(Integer::eq, this),
            "ne" => Closure::new_first(Integer::ne, this),
            _ => Error::new("get_member not implemented"),
        }
    }
}

impl Integer {
    pub fn new(value: i32) -> Object {
        Self::from_data(Integer { value })
    }


    fn add(input: Object) -> Object {
        tuple_match!(input, (a: Integer, b: Integer) {
            Integer::new(a.value + b.value)
        })
    }

    fn eq(input: Object) -> Object {
        tuple_match!(input, (a: Integer, b: Integer) {
            Bool::new(a.value == b.value)
        })
    }

    fn ne(input: Object) -> Object {
        tuple_match!(input, (a: Integer, b: Integer) {
            Bool::new(a.value != b.value)
        })
    }

    fn to_string(input: Object) -> Object {
        tuple_match!(input, (a: Integer) {
            String_::new(a.value.to_string())
        })
    } 
}

#[cfg(test)]
mod tests {
    use crate::tuple;

    use super::*;

    #[test]
    fn test_add() {
        let int1 = Integer::new(42);
        let int2 = Integer::new(1);

        let sum = int1.get_member("add").call(tuple!(int2));

        assert_eq!(
            sum.get_data::<Integer>().unwrap().value,
            43
        );
    }

    #[test]
    fn test_to_string() {
        let int = Integer::new(42);

        let str_obj = int.get_member("to_string").call(tuple!());

        assert_eq!(
            str_obj.get_data::<String_>().unwrap().value,
            "42".to_string()
        );
    }
}
