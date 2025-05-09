use std::collections::HashMap;

use crate::object::{object::Object, object_trait::ObjectTrait};

use crate::types::variable::variable::Variable;

pub struct Local<'a> {
    pub map: HashMap<&'a str, Object>,
}

impl ObjectTrait for Local<'_> {
    fn get_member_fn(this: Object, name: &str) -> Object {
        let local = this.get_data::<Local>().unwrap();
        if let Some(ret) = local.map.get(name) {
            ret.clone()
        } else {
            let var = Variable::new();
            local.map.insert(name, var.clone());
            var
        }
    }
}

impl Local<'_> {
    pub fn new() -> Object {
        Self::from_data(Local {
            map: HashMap::new(),
        })
    }
}
