use crate::object::{object_trait::ObjectTrait, object::Object};

pub struct ObjType {
    pub call: fn(Object, Object) -> Object,
    pub get_member: fn(Object, &str) -> Object,
}

impl ObjectTrait for ObjType {}

unsafe impl Sync for ObjType {}

impl ObjType {
    fn call_fn(_: Object, _: Object) -> Object {
        panic!("call not implemented")
    }

    fn get_member_fn(_: Object, _: &str) -> Object {
        panic!("get_member not implemented")
    }
}

pub static OBJ_TYPE: ObjType = ObjType {
    call: |_, _| panic!("call not implemented"),
    get_member: ObjType::get_member_fn,
};

pub static OBJ_TYPE_BOX: Object = Object {
    data: &OBJ_TYPE as *const ObjType as *mut (),
    obj_type: &OBJ_TYPE_BOX as *const Object as *mut Object,
};
