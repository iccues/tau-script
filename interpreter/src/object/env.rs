use std::{collections::HashMap, ops::Deref, rc::Rc};

use crate::object_box::ObjectBox;

use super::{variable::Variable, Object, ObjectTrait};

#[derive(Debug)]
pub struct Env {
    map: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Object {
        ObjectBox::new(Env { map: HashMap::new() })
    }
}

impl ObjectTrait for Env {
    fn to_string_row(&self) -> String {
        "env".to_string()
    }
    
    fn get_member(self: Rc<Self>, name: &str) -> Object {
        // self.map.get(name).unwrap().clone()
        if let Some(ret) = self.map.get(name) {
            ret.clone()
        } else {
            let var = Variable::new();

            #[allow(invalid_reference_casting)]
            unsafe {
                &mut *(self.deref() as *const Env as *mut Env) 
            }.map.insert(name.to_string(), var.clone());

            var
        }
    }
}
