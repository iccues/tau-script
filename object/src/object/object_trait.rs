use super::object::Object;

pub trait ObjectTrait {
    // fn as_object_box(&self) -> Object {
    //     Object::new(self as *const Self as *mut ObjHead)
    // }

    // fn into_object_box(self) -> Object
    // where
    //     Self: Sized,
    // {
    //     Object::new(Box::into_raw(Box::new(self)) as *mut ObjHead)
    // }
}
