use crate::object::prelude::*;

#[derive(Debug)]
pub struct RustFunc {
    value: fn(Object) -> Object,
}

impl ObjectTrait for RustFunc {}

impl ObjectTraitExt for RustFunc {
    const CALLABLE: bool = true;
    fn call(&mut self, input: Object) -> Object {
        (self.value)(input)
    }
}

impl RustFunc {
    pub fn new(value: fn(Object) -> Object) -> Object<RustFunc> {
        RustFunc {
            value
        }.from_data()
    }
}