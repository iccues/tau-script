use crate::matches_;
use crate::object::prelude::*;
use crate::types::callable::{closure::Closure, rust_func::RustFunc};

#[derive(Debug)]
struct IntegerType;

impl ObjectTrait for IntegerType {}

impl ObjectTraitExt for IntegerType {
    fn get_member(_this: Object<Self>, name: &str) -> Option<Object> {
        match name {
            "add" => Some(Closure::new(RustFunc::new(Integer::add), 2)),
            _ => None
        }
    }

    const MATCHABLE: bool = true;
}


#[derive(Debug, PartialEq, Eq)]
pub struct Integer {
    value: i64,
}

impl ObjectTrait for Integer {}

impl ObjectTraitExt for Integer {
    fn get_object_type() -> Option<Object> {
        Some(IntegerType.from_data())
    }
}

impl Integer {
    pub fn new(value: i64) -> Object<Integer> {
        Integer { value }.from_data()
    }

    #[cfg(test)]
    pub fn get_value(&self) -> i64 {
        self.value
    }

    fn add(input: Object) -> Object {
        matches_!((a: Integer, (b: Integer)) = input);
        Integer::new(a.value + b.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple;

    use super::*;

    #[test]
    fn test_integer_add() {
        let a: Object = Integer::new(20);
        let b: Object = Integer::new(22);
        let c = a.get_member("add").unwrap().call(tuple!(b));
        assert_eq!(c.downcast::<Integer>().unwrap().value, 42)
    }
}
