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
        Object::from_raw(Box::into_raw(Box::new(data)), obj_type)
    }

    pub const fn from_raw<T: ObjectTrait>(data: *const T, obj_type: &Object) -> Self {
        Object {
            data: data as *mut (),
            obj_type: obj_type as *const Object as *mut Object,
        }
    }

    pub unsafe fn get_data_uncheck<T: ObjectTrait>(&self) -> &mut T {
        unsafe { &mut *(self.data as *mut T) }
    }

    pub fn get_data<T: ObjectTrait>(&self) -> Option<&mut T> {
        if *self.get_obj_type() == T::OBJ_TYPE {
            Some(unsafe { self.get_data_uncheck::<T>() })
        } else {
            None
        }
    }

    pub fn get_data_match<T: ObjectTrait>(&self) -> Option<&mut T> {
        if let Some(data) = (T::OBJ_TYPE_BOX.get_obj_type().match_)(T::OBJ_TYPE_BOX, self.clone()) {
            Some(unsafe { transmute::<&mut T, &mut T>(data.get_data_uncheck::<T>()) })
        } else {
            None
        }
    }

    pub fn get_obj_type(&self) -> &ObjType {
        unsafe { (*self.obj_type).get_data_uncheck::<ObjType>() }
    }

    pub fn get_member(&self, name: &str) -> Object {
        (self.get_obj_type().get_member)(self.clone(), name)
    }

    pub fn call(&self, input: Object) -> Object {
        (self.get_obj_type().call)(self.clone(), input)
    }

    pub fn match_(&self, other: Object) -> Option<Object> {
        (self.get_obj_type().match_)(self.clone(), other)
    }

    pub fn on_matched(&self, other: Object) -> Object {
        (self.get_obj_type().on_matched)(self.clone(), other)
    }

    pub fn drop(&mut self) {
        if self.data.is_null() {
            return;
        }
        (self.get_obj_type().drop)(self.clone());
        unsafe {
            let _ = Box::from_raw(self.data);
        };
        self.data = std::ptr::null_mut();
    }
}

#[cfg(test)]
mod tests {
    use crate::types::primitive::number::Integer;

    #[test]
    fn test_drop() {
        let mut obj = Integer::new(42);
        obj.drop();
    }
}
