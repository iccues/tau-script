use std::cell::RefCell;

use object_core::prelude::*;
use object_ext::object_ext::ObjectExt;
use object_ext::tuple;
use object_ext::{matches_, object_trait_ext::ObjectTraitExt};
use object_ext::core_type::string::ObjString;
use object_ext::core_type::tuple::Tuple;
use object_ext::core_type::callable::{closure::Closure, rust_func::RustFunc};

use crate::primitive::numbers::ObjI64;

#[derive(Debug, ObjectTrait)]
pub struct ObjListTypeType;

impl ObjectTraitExt for ObjListTypeType {
    fn get_member(_this: Object<Self>, name: &str) -> Option<Object> {
        match name {
            "to_string" => Some(Closure::new(RustFunc::new(ObjList::to_string), 3)),
            "insert" => Some(Closure::new(RustFunc::new(ObjList::insert), 3)),
            _ => None,
        }
    }

    const CALLABLE: bool = true;
    fn call(_this: Object<Self>, input: Object) -> Object {
        matches_!((list_type) = input);
        ObjListType::new(_this, list_type)
    }
}


#[derive(Debug, ObjectTrait)]
pub struct ObjListType {
    pub type_: Object,
}

impl ObjectTraitExt for ObjListType {
    fn get_object_type() -> Option<Object> {
        Some(ObjListTypeType.from_data())
    }

    const CALLABLE: bool = true;
    fn call(this: Object<Self>, input: Object) -> Object {
        matches_!(list: Tuple = input);
        let elements = list.get_vec();
        for ele in elements.clone() {
            if this.type_.match_(ele).is_none() {
                panic!("Unmatched input");
            }
        }
        ObjList::new(this, RefCell::new(elements))
    }
}

impl ObjListType {
    fn new(list_type_type: Object, input: Object) -> Object {
        matches_!(type_ = input);
        Self::from_data_type(
            ObjListType { type_ },
            Some(list_type_type)
        )
    }
}


#[derive(Debug, ObjectTrait)]
pub struct ObjList {
    pub elements: RefCell<Vec<Object>>,
}

impl ObjectTraitExt for ObjList {}

impl ObjList {
    fn new(list_type: Object, elements: RefCell<Vec<Object>>) -> Object {
        Self::from_data_type(
            ObjList { elements },
            Some(list_type)
        )
    }

    fn to_string(input: Object) -> Object {
        matches_!((_list_type, list: ObjList, ()) = input);
        let elements: Vec<String> = list.elements.borrow().iter()
            .map(|e| e.object_to_string())
            .collect();
        ObjString::new(format!("[{}]", elements.join(", ")))
    }

    fn insert(input: Object) -> Object {
        matches_!((list_type: ObjListType, list: ObjList, (index: ObjI64, element)) = input);
        if list_type.type_.match_(element.clone()).is_none() {
                panic!("Unmatched input");
            }
        list.elements.borrow_mut().insert(index.value as usize, element);
        tuple!()
    }
}
