use std::{any::Any, fmt::Debug, rc::Rc};

use downcast_rs::{impl_downcast, Downcast};
use undefined::Undefined;

use crate::object_box::ObjectBox;

pub mod string;
pub mod number;
pub mod object_fn;
pub mod tuple;
pub mod env;
pub mod closure;
pub mod variable;
pub mod undefined;

pub type Object = ObjectBox<dyn ObjectTrait>;

pub trait ObjectTrait: Downcast + Debug + Any {
    fn to_string_row(&self) -> String;
    fn get_member(self: Rc<Self>, name: &str) -> Object {
        let _ = name;
        Undefined::new()
    }
    fn call(&self, input: Object) -> Object {
        let _ = input;
        panic!("Not callable");
    }
}

impl_downcast!(ObjectTrait);


#[cfg(test)]
mod tests {
    use crate::object::number::Integer;
    use crate::object::tuple::Tuple;

    #[test]
    fn test_integer() {
        let integer = Integer::new("42");
        assert_eq!(integer.to_string_row(), "42");
    }

    #[test]
    fn test_tuple() {
        let tuple = Tuple::new(vec![
            Integer::new("42"),
            Integer::new("24"),
        ]);
        assert_eq!(tuple.to_string_row(), "(42, 24)");
    }

    #[test]
    fn test_integer_add() {
        let integer = Integer::new("42");
        let result = integer.get_member("add").call(Tuple::new(vec![
            Integer::new("24"),
        ]));
        assert_eq!(result.to_string_row(), "66");
    }
}