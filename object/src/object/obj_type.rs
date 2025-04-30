use crate::object::{object_trait::ObjectTrait, object::Object};

pub struct ObjType {
    pub call: fn(Object, Object) -> Object,
    pub get_member: fn(Object, &str) -> Object,
}

impl ObjectTrait for ObjType {}

impl ObjType {
    fn call_fn(_: Object, _: Object) -> Object {
        panic!("call not implemented")
    }

    fn get_member_fn(_: Object, _: &str) -> Object {
        panic!("get_member not implemented")
    }
}

static OBJ_TYPE: ObjType = ObjType {
    call: ObjType::call_fn,
    get_member: ObjType::get_member_fn,
};

pub static OBJ_TYPE_BOX: Object = Object::from_raw(&OBJ_TYPE, &OBJ_TYPE_BOX);