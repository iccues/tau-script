use object_core::prelude::{Object, ObjectTrait};
use object_ext::object_trait_ext::ObjectTraitExt;

use crate::register_build_in;

#[derive(Debug, ObjectTrait)]
pub struct Any;

impl ObjectTraitExt for Any {
    const MATCHABLE: bool = true;

    fn match_(_this: Object<Self>, input: Object) -> Option<Object> {
        Some(input)
    }
}

impl Any {
    pub fn new() -> Object<Any> {
        Any.from_data()
    }
}

register_build_in!("any", Any::new());
