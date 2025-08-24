use object_core::prelude::*;
use object_ext::core_type::string::ObjString;
use object_ext::object_trait_ext::ObjectTraitExt;
use object_ext::matches_;
use object_ext::core_type::callable::{closure::Closure, rust_func::RustFunc};

#[derive(Debug)]
struct ObjBoolType;

impl ObjectTrait for ObjBoolType {}

impl ObjectTraitExt for ObjBoolType {
    fn get_member(_this: Object<Self>, name: &str) -> Option<Object> {
        match name {
            "to_string" => Some(Closure::new(RustFunc::new(ObjBool::to_string), 2)),
            _ => None,
        }
    }

    const MATCHABLE: bool = true;
}

#[derive(Debug)]
pub struct ObjBool {
    pub value: bool,
}

impl ObjectTrait for ObjBool {}

impl ObjectTraitExt for ObjBool {
    fn get_object_type() -> Option<Object> {
        Some(ObjBoolType.from_data())
    }
}

impl ObjBool {
    pub fn new(value: bool) -> Object<ObjBool> {
        ObjBool { value }.from_data()
    }

    fn to_string(input: Object) -> Object{
        matches_!((a: ObjBool, ()) = input);
        ObjString::new(a.value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use object_ext::tuple;

    use super::*;

    #[test]
    fn test_bool_to_string() {
        let a: Object = ObjBool::new(true);
        let c = a.get_member("to_string").unwrap().call(tuple!());
        assert_eq!(c.downcast::<ObjString>().unwrap().value, "true");

        let b: Object = ObjBool::new(false);
        let d = b.get_member("to_string").unwrap().call(tuple!());
        assert_eq!(d.downcast::<ObjString>().unwrap().value, "false");
    }
}
