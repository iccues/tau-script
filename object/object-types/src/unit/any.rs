use object_core::prelude::{Object, ObjectTrait};
use object_ext::object_trait_ext::ObjectTraitExt;

#[derive(Debug, ObjectTrait)]
pub struct ObjAny;

impl ObjectTraitExt for ObjAny {
    const MATCHABLE: bool = true;

    fn match_(_this: Object<Self>, input: Object) -> Option<Object> {
        Some(input)
    }
}

impl ObjAny {
    pub fn new() -> Object<ObjAny> {
        ObjAny.from_data()
    }
}
