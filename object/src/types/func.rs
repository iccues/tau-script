use core::panic;

use crate::object::{obj_type::{ObjType, OBJ_TYPE_BOX}, object::Object, object_trait::ObjectTrait};

pub struct Func {
    func: Box<dyn Fn(Object) -> Object>,
}

impl ObjectTrait for Func {}

impl Func {
    pub fn new<F>(func: F) -> Object
    where
        F: Fn(Object) -> Object + 'static,
    {
        Object::new(
            Func { func: Box::new(func) },
            &FUNC_TYPE_BOX
        )
    }

    pub fn call_fn(func: Object, input: Object) -> Object {
        let func: &Func = func.get_data_as::<Func>();
        (func.func)(input)
    }
}

pub static FUNC_TYPE: ObjType = ObjType {
    call: Func::call_fn,
    get_member: |_, _| panic!("get_member not implemented"),
};

pub static FUNC_TYPE_BOX: Object = Object::from_raw(&FUNC_TYPE, &OBJ_TYPE_BOX);
