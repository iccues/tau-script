use std::mem::transmute;

use super::{obj_type::ObjType, object_trait::ObjectTrait};

#[derive(Clone)]
pub struct Object {
    pub data: *mut (),
    pub obj_type: *mut Object,
}

unsafe impl Sync for Object {}
unsafe impl Send for Object {}

impl Object {
    pub fn new<T: ObjectTrait>(data: T, obj_type: &Object) -> Self {
        Object {
            data: Box::into_raw(Box::new(data)) as *mut (),
            obj_type: obj_type as *const Object as *mut Object,
        }
    }

    pub const fn from_raw<T: ObjectTrait + 'static>(data: &T, obj_type: &Object) -> Self {
        Object {
            data: data as *const T as *mut (),
            obj_type: obj_type as *const Object as *mut Object,
        }
    }

    pub unsafe fn get_data_uncheck<T: ObjectTrait>(&self) -> &mut T { // unsafe
        unsafe { &mut *(self.data as *mut T) }
    }

    pub fn get_data<T: ObjectTrait>(&self) -> Option<&mut T> {
        if let Some(data) = (T::OBJ_TYPE_BOX.get_obj_type().match_)(T::OBJ_TYPE_BOX, self.clone()) {
            Some(unsafe { transmute(data.get_data_uncheck::<T>()) })
        } else {
            None
        }
    }

    pub fn get_obj_type(&self) -> &ObjType {
        unsafe { (&*self.obj_type).get_data_uncheck::<ObjType>() }
    }

    pub fn get_member(&self, name: &str) -> Object {
        (self.get_obj_type().get_member)(self.clone(), name)
    }

    pub fn call(&self, input: Object) -> Object {
        (self.get_obj_type().call)(self.clone(), input)
    }
}
