use std::collections::HashMap;

use crate::object::{object::Object, object_trait::ObjectTrait};

use crate::types::variable::variable::Variable;

pub struct Local {
    pub map: HashMap<String, Object>,
}

impl ObjectTrait for Local {
    fn get_member_fn(this: Object, name: &str) -> Object {
        let local = this.get_data::<Local>().unwrap();
        if let Some(ret) = local.map.get(name) {
            ret.clone()
        } else {
            let var = Variable::new();
            local.map.insert(name.to_string(), var.clone());
            var
        }
    }
}

impl Local {
    pub fn new() -> Object {
        Self::from_data(Local {
            map: HashMap::new(),
        })
    }
}
