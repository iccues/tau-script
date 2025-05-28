use std::collections::HashMap;

use crate::object::{object::Object, object_trait::ObjectTrait};

use crate::types::control::undefined::Undefined;
use crate::types::variable::variable::Variable;

pub struct Local {
    map: HashMap<String, Object>,
    outers: Vec<Object>,
}

impl ObjectTrait for Local {
    fn get_member_fn(this: Object, name: &str) -> Object {
        let local = this.get_data::<Local>().unwrap();
        if let Some(ret) = local.map.get(name) {
            ret.clone()
        } else {
            for outer in local.outers.iter() {
                let ret = outer.get_member(name);
                if Undefined::OBJ_TYPE_BOX.match_(ret.clone()).is_none() {
                    return ret;
                }
            }

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
            outers: Vec::new(),
        })
    }

    pub fn from_outer(outers: Vec<Object>) -> Object {
        Self::from_data(Local {
            map: HashMap::new(),
            outers,
        })
    }
}
