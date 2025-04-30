use super::{obj_type::{ObjType, OBJ_TYPE_BOX}, object::Object};

pub trait ObjectTrait {
    const OBJ_TYPE: ObjType = ObjType {
        call: Self::call_fn,
        get_member: Self::get_member_fn,
    };
    const OBJ_TYPE_TYPE: &Object = &OBJ_TYPE_BOX;

    const OBJ_TYPE_BOX: Object = Object::from_raw(&Self::OBJ_TYPE, Self::OBJ_TYPE_TYPE);

    fn get_member_fn(this: Object, name: &str) -> Object {
        let _ = (this, name);
        panic!("get_member not implemented")
    }
    fn call_fn(this: Object, args: Object) -> Object {
        let _ = (this, args);
        panic!("call not implemented")
    }

    fn from_data(data: Self) -> Object
    where
        Self: Sized,
    {
        Object::new(data, &Self::OBJ_TYPE_BOX)
    }
}
