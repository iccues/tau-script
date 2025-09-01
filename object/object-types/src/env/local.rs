use std::{cell::RefCell, collections::HashMap};

use object_core::prelude::*;
use object_ext::object_trait_ext::ObjectTraitExt;
use crate::variable::Variable;

#[derive(Debug, ObjectTrait)]
pub struct ObjLocal {
    map: RefCell<HashMap<String, Object>>,
    outer: Option<Object>,
}

impl ObjectTraitExt for ObjLocal {
    fn get_member(this: Object<Self>, name: &str) -> Option<Object> {
        if let Some(ret) = this.map.borrow().get(name) {
            Some(ret.clone())
        } else {
            if let Some(outer) = &this.outer {
                if let Ok(ret) = outer.get_member(name) {
                    return Some(ret);
                }
            }
            let var = Variable::new();
            this.map.borrow_mut().insert(name.to_string(), var.clone());
            Some(var)
        }
    }
}

impl ObjLocal {
    pub fn new() -> Object<Self> {
        Self::from_data(Self {
            map: RefCell::new(HashMap::new()),
            outer: None,
        })
    }

    pub fn from_outer(outer: Object) -> Object<Self> {
        Self::from_data(Self {
            map: RefCell::new(HashMap::new()),
            outer: Some(outer),
        })
    }
}
