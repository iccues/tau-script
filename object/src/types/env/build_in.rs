use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{object::{object::Object, object_trait::ObjectTrait}, types::control::undefined::Undefined};

pub struct BuildIn {
    map: HashMap<String, Object>,
}

impl ObjectTrait for BuildIn {
    fn get_member_fn(this: Object, name: &str) -> Object {
        let local = this.get_data::<BuildIn>().unwrap();
        if let Some(ret) = local.map.get(name) {
            ret.clone()
        } else {
            Undefined::new()
        }
    }
}

impl BuildIn {
    pub fn new() -> Object {
        BUILD_IN.clone()
    }

    pub fn insert(key: String, value: Object) {
        let build_in = BUILD_IN.get_data::<BuildIn>().unwrap();
        build_in.map.insert(key, value);
    }
}

static BUILD_IN: Lazy<Object> = Lazy::new(|| {
    BuildIn::from_data(BuildIn { map: HashMap::new() })
});
