use object_core::prelude::*;
use object_ext::object_trait_ext::ObjectTraitExt;

#[derive(Debug)]
pub struct Undefined;

impl ObjectTrait for Undefined {}

impl ObjectTraitExt for Undefined {
    
}

impl Undefined {
    pub fn new() -> Object<Undefined> {
        Undefined.from_data()
    }
}
