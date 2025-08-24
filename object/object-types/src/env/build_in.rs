use std::{cell::RefCell, collections::HashMap};

use object_core::prelude::*;
use object_ext::object_trait_ext::ObjectTraitExt;
// use once_cell::sync::Lazy;

use crate::{list::ListTypeType, primitive::numbers::ObjI64Type};

#[derive(Debug)]
pub struct BuildIn {
    map: RefCell<HashMap<String, Object>>,
}

impl ObjectTrait for BuildIn {}

impl ObjectTraitExt for BuildIn {
    fn get_member(this: Object<Self>, name: &str) -> Option<Object> {
        if let Some(ret) = this.map.borrow().get(name) {
            Some(ret.clone())
        } else {
            None
        }
    }
}

impl BuildIn {
    pub fn new() -> Object<Self> {
        let mut map: HashMap<String, Object> = HashMap::new();
        map.insert("list_type".to_string(), ListTypeType.from_data());
        map.insert("int".to_string(), ObjI64Type.from_data());

        BuildIn { map: RefCell::new(map), }.from_data()
    }
}

// static BUILD_IN: Lazy<Object<BuildIn>> = Lazy::new(|| {
//     let mut map: HashMap<String, Object> = HashMap::new();
//     map.insert("list_type".to_string(), ListTypeType.from_data());

//     BuildIn { map: RefCell::new(map), }.from_data()
// });
