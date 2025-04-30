use crate::object::{object::Object, object_trait::ObjectTrait};

use super::func::Func;

pub struct Integer {
    pub value: i32,
}

impl ObjectTrait for Integer {
    fn get_member_fn(_this: Object, name: &str) -> Object {
        match name {
            "incr" => Func::new(Integer::incr),
            _ => panic!("get_member not implemented"),
        }
    }
}

impl Integer {
    pub fn new(value: i32) -> Object {
        Self::from_data(Integer { value })
    }

    fn incr(input: Object) -> Object {
        let integer = input.get_data_as::<Integer>();
        let result = integer.value + 1; // Example operation
        Integer::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let integer = Integer::new(42);
        let incr_int = integer.get_member_("incr").call_(integer.clone());
        assert_eq!(
            incr_int.get_data_as::<Integer>().value,
            43
        );
    }
}
