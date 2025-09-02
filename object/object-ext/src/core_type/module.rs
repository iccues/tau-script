use std::collections::HashMap;

use object_core::prelude::*;

use crate::object_trait_ext::ObjectTraitExt;

#[derive(Debug, ObjectTrait)]
pub struct Module {
    map: HashMap<String, Object>,
}

impl ObjectTraitExt for Module {
    fn get_member(this: Object<Self>, name: &str) -> Option<Object> {
        this.map.get(name).cloned()
    }
}

impl Module {
    pub fn new() -> Object<Self> {
        Self::from_data(Self {
            map: HashMap::new(),
        })
    }

    pub fn from_map(map: HashMap<String, Object>) -> Object<Self> {
        Self::from_data(Self { map })
    }

    pub fn insert(&mut self, name: &str, obj: Object) {
        self.map.insert(name.to_string(), obj);
    }
}

#[macro_export]
macro_rules! module {
    ($($name:ident = $obj:expr);* ;) => {
        std::sync::LazyLock::new(|| {
            #[allow(unused_mut)]
            let mut map = std::collections::HashMap::<String, object_core::prelude::Object>::new();
            $(
                map.insert(stringify!($name).to_string(), $obj);
            )*
            $crate::core_type::module::Module::from_map(map)
        })
    };
}
