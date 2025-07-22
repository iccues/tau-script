use object_core::prelude::*;
use object_ext::object_trait_ext::ObjectTraitExt;

#[derive(Debug)]
pub struct RustFunc {
    value: fn(Object) -> Object,
}

impl ObjectTrait for RustFunc {}

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