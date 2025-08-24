use std::cell::RefCell;

use object_core::prelude::*;
use object_ext::object_ext::ObjectExt;
use object_ext::tuple;
use object_ext::{matches_, object_trait_ext::ObjectTraitExt};
use object_ext::core_type::string::ObjString;
use object_ext::core_type::tuple::Tuple;
use object_ext::core_type::callable::{closure::Closure, rust_func::RustFunc};

use crate::primitive::numbers::ObjI64;

#[derive(Debug)]
pub struct ListTypeType;

impl ObjectTrait for ListTypeType {}

impl ObjectTraitExt for ListTypeType {
    fn get_member(_this: Object<Self>, name: &str) -> Option<Object> {
        match name {
            "to_string" => Some(Closure::new(RustFunc::new(List::to_string), 3)),
            "insert" => Some(Closure::new(RustFunc::new(List::insert), 3)),
            _ => None,
        }
    }

    const CALLABLE: bool = true;
    fn call(_this: Object<Self>, input: Object) -> Object {
        matches_!((list_type) = input);
        ListType::new(_this, list_type)
    }
}


#[derive(Debug)]
pub struct ListType {
    pub type_: Object,
}

impl ObjectTrait for ListType {}

impl ObjectTraitExt for ListType {
    fn get_object_type() -> Option<Object> {
        Some(ListTypeType.from_data())
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
        List::new(this, RefCell::new(elements))
    }
}

impl ListType {
    fn new(list_type_type: Object, input: Object) -> Object {
        matches_!(type_ = input);
        Self::from_data_type(
            ListType { type_ },
            Some(list_type_type)
        )
    }
}


#[derive(Debug)]
pub struct List {
    pub elements: RefCell<Vec<Object>>,
}

impl ObjectTrait for List {}

impl ObjectTraitExt for List {}

impl List {
    fn new(list_type: Object, elements: RefCell<Vec<Object>>) -> Object {
        Self::from_data_type(
            List { elements },
            Some(list_type)
        )
    }

    fn to_string(input: Object) -> Object {
        matches_!((_list_type, list: List, ()) = input);
        let elements: Vec<String> = list.elements.borrow().iter()
            .map(|e| e.object_to_string())
            .collect();
        ObjString::new(format!("[{}]", elements.join(", ")))
    }

    fn insert(input: Object) -> Object {
        matches_!((list_type: ListType, list: List, (index: ObjI64, element)) = input);
        if list_type.type_.match_(element.clone()).is_none() {
                panic!("Unmatched input");
            }
        list.elements.borrow_mut().insert(index.value as usize, element);
        tuple!()
    }
}
