use crate::{matches_, object::prelude::*, types::callable::{closure::Closure, rust_func::RustFunc}};

#[derive(Debug)]
struct ObjStringType;

impl ObjectTrait for ObjStringType {}

impl ObjectTraitExt for ObjStringType {
    fn get_member(_this: Object<Self>, name: &str) -> Option<Object> {
        match name {
            "to_string" => Some(Closure::new(RustFunc::new(ObjString::to_string), 2)),
            _ => None,
        }
    }

    const MATCHABLE: bool = true;
}

#[derive(Debug)]
pub struct ObjString {
    pub value: String,
}

impl ObjectTrait for ObjString {}

impl ObjectTraitExt for ObjString {
    fn get_object_type() -> Option<Object> {
        Some(ObjStringType.from_data())
    }
}

impl ObjString {
    pub fn new(value: String) -> Object<ObjString> {
        ObjString { value }.from_data()
    }

    fn to_string(input: Object) -> Object {
        matches_!((a: ObjString, ()) = input);
        ObjString::new(a.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple;

    use super::*;

    #[test]
    fn test_string_to_string() {
        let a: Object = ObjString::new("Hello".to_string());
        let c = a.get_member("to_string").unwrap().call(tuple!());
        assert_eq!(c.downcast::<ObjString>().unwrap().value, "Hello");
    }
}
