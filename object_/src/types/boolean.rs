use crate::object::prelude::{Object, ObjectTrait, ObjectTraitExt};

#[derive(Debug)]
pub struct Boolean {
    pub value: bool,
}

impl ObjectTrait for Boolean {}

impl ObjectTraitExt for Boolean {
    
}

impl Boolean {
    pub fn new(value: bool) -> Object<Boolean> {
        Boolean { value }.from_data()
    }
}
