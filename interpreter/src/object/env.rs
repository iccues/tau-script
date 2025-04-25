use std::collections::HashMap;

use crate::object_box::ObjectBox;

use super::{ObjectTrait, Object};

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
    
    fn get_member(&self, name: &str) -> Object {
        self.map.get(name).unwrap().clone()
    }
}
