use crate::object::prelude::*;

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
