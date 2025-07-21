use crate::object::{object::Object, prelude::ObjectTrait};

#[derive(Debug)]
pub struct ObjectVTable<T: ObjectTrait> {
    pub get_member_fn: fn(this: Object<T>, &str) -> Option<Object>,
    pub call_fn: Option<fn(this: Object<T>, Object) -> Object>,
    pub match_fn: Option<fn(this: Object<T>, Object) -> Option<Object>>,
}
