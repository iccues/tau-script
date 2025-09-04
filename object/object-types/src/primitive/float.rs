use object_core::prelude::*;
use object_ext::{core_type::{callable::{closure::Closure, rust_func::RustFunc}, string::ObjString}, matches_, object_trait_ext::ObjectTraitExt};

use crate::primitive::bool::ObjBool;

#[derive(Debug, ObjectTrait)]
pub struct ObjF64Type;

impl ObjectTraitExt for ObjF64Type {
    fn get_member(_this: Object<Self>, name: &str) -> Option<Object> {
        match name {
            "add" => Some(Closure::new(RustFunc::new(ObjF64::add), 2)),
            "to_string" => Some(Closure::new(RustFunc::new(ObjF64::to_string), 2)),
            "eq" => Some(Closure::new(RustFunc::new(ObjF64::eq), 2)),
            "ne" => Some(Closure::new(RustFunc::new(ObjF64::ne), 2)),
            _ => None
        }
    }

    const MATCHABLE: bool = true;
}


#[derive(Debug, ObjectTrait, PartialEq)]
pub struct ObjF64 {
    pub value: f64,
}

impl ObjectTraitExt for ObjF64 {
    fn get_object_type() -> Option<Object> {
        Some(ObjF64Type.from_data())
    }
}

impl ObjF64 {
    pub fn new(value: f64) -> Object<ObjF64> {
        ObjF64 { value }.from_data()
    }

    fn add(input: Object) -> Object {
        matches_!((a: ObjF64, (b: ObjF64)) = input);
        ObjF64::new(a.value + b.value)
    }

    fn to_string(input: Object) -> Object {
        matches_!((a: ObjF64, ()) = input);
        ObjString::new(a.value.to_string())
    }

    fn eq(input: Object) -> Object {
        matches_!((a: ObjF64, (b: ObjF64)) = input);
        ObjBool::new(a.value == b.value)
    }

    fn ne(input: Object) -> Object {
        matches_!((a: ObjF64, (b: ObjF64)) = input);
        ObjBool::new(a.value != b.value)
    }
}