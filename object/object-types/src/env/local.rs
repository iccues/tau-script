use std::{cell::RefCell, collections::HashMap};

use object_core::prelude::*;
use object_ext::object_trait_ext::ObjectTraitExt;
use crate::variable::Variable;

#[derive(Debug, ObjectTrait)]
pub struct ObjLocal {
    map: RefCell<HashMap<String, Object>>,
    prelude: Option<Object>,
}

impl ObjectTraitExt for ObjLocal {
    fn get_member(this: Object<Self>, name: &str) -> Option<Object> {
        if let Some(ret) = this.map.borrow().get(name) {
            Some(ret.clone())
        } else {
            if let Some(prelude) = &this.prelude {
                if let Ok(ret) = prelude.get_member(name) {
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
            prelude: None,
        })
    }

    pub fn from_prelude(prelude: Object) -> Object<Self> {
        Self::from_data(Self {
            map: RefCell::new(HashMap::new()),
            prelude: Some(prelude),
        })
    }
}
