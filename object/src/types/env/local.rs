use std::{cell::RefCell, collections::HashMap};

use crate::{object::prelude::*, types::variable::Variable};

#[derive(Debug)]
pub struct ObjLocal {
    map: RefCell<HashMap<String, Object>>,
}

impl ObjectTrait for ObjLocal {}

impl ObjectTraitExt for ObjLocal {
    fn get_member(this: Object<Self>, name: &str) -> Option<Object> {
        if let Some(ret) = this.map.borrow().get(name) {
            Some(ret.clone())
        } else {
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
        })
    }
}
