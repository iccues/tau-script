use object_core::prelude::*;
use object_ext::object_trait_ext::ObjectTraitExt;

#[derive(Debug)]
pub struct ObjError {
    pub message: String,
}

impl ObjectTrait for ObjError {}

impl ObjectTraitExt for ObjError {
    
}

impl ObjError {
    pub fn new(message: String) -> Object<Self> {
        Self{ message }.from_data()
    }
}
