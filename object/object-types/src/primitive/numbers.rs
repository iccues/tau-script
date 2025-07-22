use object_core::prelude::*;
use object_ext::object_trait_ext::ObjectTraitExt;
use object_ext::matches_;
use crate::callable::{closure::Closure, rust_func::RustFunc};
use crate::primitive::bool::ObjBool;
use crate::primitive::string::ObjString;

#[derive(Debug)]
struct ObjI64Type;

impl ObjectTrait for ObjI64Type {}

impl ObjectTraitExt for ObjI64Type {
    fn get_member(_this: Object<Self>, name: &str) -> Option<Object> {
        match name {
            "add" => Some(Closure::new(RustFunc::new(ObjI64::add), 2)),
            "to_string" => Some(Closure::new(RustFunc::new(ObjI64::to_string), 2)),
            "eq" => Some(Closure::new(RustFunc::new(ObjI64::eq), 2)),
            "ne" => Some(Closure::new(RustFunc::new(ObjI64::ne), 2)),
            _ => None
        }
    }

    const MATCHABLE: bool = true;
}


#[derive(Debug, PartialEq, Eq)]
pub struct ObjI64 {
    pub value: i64,
}

impl ObjectTrait for ObjI64 {}

impl ObjectTraitExt for ObjI64 {
    fn get_object_type() -> Option<Object> {
        Some(ObjI64Type.from_data())
    }
}

impl ObjI64 {
    pub fn new(value: i64) -> Object<ObjI64> {
        ObjI64 { value }.from_data()
    }

    fn add(input: Object) -> Object {
        matches_!((a: ObjI64, (b: ObjI64)) = input);
        ObjI64::new(a.value + b.value)
    }

    fn to_string(input: Object) -> Object {
        matches_!((a: ObjI64, ()) = input);
        ObjString::new(a.value.to_string())
    }

    fn eq(input: Object) -> Object {
        matches_!((a: ObjI64, (b: ObjI64)) = input);
        ObjBool::new(a.value == b.value)
    }

    fn ne(input: Object) -> Object {
        matches_!((a: ObjI64, (b: ObjI64)) = input);
        ObjBool::new(a.value != b.value)
    }
}

#[cfg(test)]
mod tests {
    use object_ext::tuple;

    use super::*;

    #[test]
    fn test_integer_add() {
        let a: Object = ObjI64::new(20);
        let b: Object = ObjI64::new(22);
        let c = a.get_member("add").unwrap().call(tuple!(b));
        assert_eq!(c.downcast::<ObjI64>().unwrap().value, 42)
    }

    #[test]
    fn test_integer_to_string() {
        let a: Object = ObjI64::new(42);
        let c = a.get_member("to_string").unwrap().call(tuple!());
        assert_eq!(c.downcast::<ObjString>().unwrap().value, "42")
    }

    #[test]
    fn test_integer_eq() {
        let a: Object = ObjI64::new(42);
        let b: Object = ObjI64::new(42);
        let c: Object = ObjI64::new(43);
        let result1 = a.get_member("eq").unwrap().call(tuple!(b));
        assert!(result1.downcast::<ObjBool>().unwrap().value);

        let result2 = a.get_member("eq").unwrap().call(tuple!(c));
        assert!(!result2.downcast::<ObjBool>().unwrap().value);
    }

    #[test]
    fn test_integer_ne() {
        let a: Object = ObjI64::new(42);
        let b: Object = ObjI64::new(42);
        let c: Object = ObjI64::new(43);
        let result1 = a.get_member("ne").unwrap().call(tuple!(b));
        assert!(!result1.downcast::<ObjBool>().unwrap().value);

        let result2 = a.get_member("ne").unwrap().call(tuple!(c));
        assert!(result2.downcast::<ObjBool>().unwrap().value);
    }
}
