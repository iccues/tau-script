use std::collections::HashMap;

use crate::object::{object::Object, object_trait::ObjectTrait};

use super::variable::Variable;

pub struct Env {
    pub map: HashMap<String, Object>,
}

impl ObjectTrait for Env {
    fn get_member_fn(this: Object, name: &str) -> Object {
        let env = this.get_data::<Env>().unwrap();
        if let Some(ret) = env.map.get(name) {
            ret.clone()
        } else {
            let var = Variable::new();
            env.map.insert(name.to_string(), var.clone());
            var
        }
    }
}

impl Env {
    pub fn new() -> Object {
        Self::from_data(Env {
            map: HashMap::new(),
        })
    }
}
