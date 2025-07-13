use crate::object::object::Object;

#[derive(Debug)]
pub struct ObjectVTable<T> {
    pub get_member_fn: fn(this: &mut T, &str) -> Option<Object>,
    pub call_fn: Option<fn(this: &mut T, Object) -> Object>,
}
