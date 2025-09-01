use object_core::prelude::*;
use crate::object_trait_ext::ObjectTraitExt;

#[derive(Debug, ObjectTrait)]
pub struct RustFunc {
    value: fn(Object) -> Object,
}

impl ObjectTraitExt for RustFunc {
    const CALLABLE: bool = true;
    fn call(this: Object<Self>, input: Object) -> Object {
        (this.value)(input)
    }
}

impl RustFunc {
    pub fn new(value: fn(Object) -> Object) -> Object<RustFunc> {
        RustFunc {
            value
        }.from_data()
    }
}
