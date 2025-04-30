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

    pub fn get_data_as<T: ObjectTrait>(&self) -> &mut T {
        unsafe { &mut *(self.data as *mut T) }
    }

    pub fn get_obj_type(&self) -> &ObjType {
        unsafe { (&*self.obj_type).get_data_as::<ObjType>() }
    }

    pub fn get_member_(&self, name: &str) -> Object {
        (self.get_obj_type().get_member)(self.clone(), name)
    }

    pub fn call_(&self, input: Object) -> Object {
        (self.get_obj_type().call)(self.clone(), input)
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        // self.get_obj_head().ref_count_sub();
    }
}
