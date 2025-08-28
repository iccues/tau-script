use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use object_core::prelude::*;
use object_ext::object_trait_ext::ObjectTraitExt;

#[derive(Debug)]
pub struct BuildIn {
    map: RwLock<HashMap<String, Object>>,
}

impl ObjectTrait for BuildIn {}

impl ObjectTraitExt for BuildIn {
    fn get_member(this: Object<Self>, name: &str) -> Option<Object> {
        if let Some(ret) = this.map.read().unwrap().get(name) {
            Some(ret.clone())
        } else {
            None
        }
    }
}

impl BuildIn {
    pub fn new() -> Object<Self> {
        BUILD_IN.clone()
    }

    pub fn insert(&self, name: String, obj: Object) {
        self.map.write().unwrap().insert(name, obj);
    }
}

pub static BUILD_IN: LazyLock<Object<BuildIn>> = LazyLock::new(|| BuildIn { map: RwLock::new(HashMap::new())}.from_data());

#[macro_export]
macro_rules! register_build_in {
    ($name:expr, $obj:expr) => {
        #[ctor::ctor]
        fn register() {
            $crate::env::build_in::BUILD_IN.insert($name.to_string(), $obj);
        }
    };
}
