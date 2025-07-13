use std::{mem::transmute, ops::Deref, rc::Rc};
use crate::tuple;
use crate::object::{object_trait::ObjectTrait, object_vtable::ObjectVTable};

pub type Object<T = dyn ObjectTrait> = Rc<ObjectInner<T>>;

#[derive(Debug)]
pub struct ObjectInner<T: ObjectTrait + ?Sized = dyn ObjectTrait> {
    object_type: Option<Object>,
    object_vtable: ObjectVTable<()>,
    object: T,
}

impl<T: ObjectTrait> ObjectInner<T> {
    pub fn new(object: T, object_vtable: ObjectVTable<T>, object_type: Option<Object>) -> Object<T> {
        Rc::new(ObjectInner {
            object_type,
            object_vtable: unsafe { transmute(object_vtable) },
            object,
        })
    }

    pub fn get_member(self: &Rc<Self>, name: &str) -> Option<Object> {
        (self.object_vtable.get_member_fn)(self.get_object_row(), name)
            .or_else(|| self.object_type.as_ref()?.get_member(name).map(|m|
                m.call(tuple!(self.clone()))))
    }
}

impl<T: ObjectTrait + ?Sized> ObjectInner<T> {
    pub fn call(self: &Rc<Self>, input: Object) -> Object {
        // if let Some(call_fn) = self.object_vtable.call_fn {
        //     call_fn(self.get_object_row(), input)
        // } else {
        //     self.get_member("call").unwrap().call(input)
        // }
        (self.object_vtable.call_fn.unwrap())(self.get_object_row(), input)
    }

    fn is<U: ObjectTrait>(&self) -> bool {
        self.object.type_id() == std::any::TypeId::of::<U>()
    }

    fn get_object_row(&self) -> &mut () {
        #[allow(invalid_reference_casting)]
        unsafe { &mut *(&self.object as *const T as *mut ()) }
    }
}

impl ObjectInner<dyn ObjectTrait> {
    pub fn downcast<T: ObjectTrait>(self: Rc<Self>) -> Result<Object<T>, Rc<Self>> {
        if self.is::<T>() {
            Ok(unsafe {
                Rc::from_raw(Rc::into_raw(self) as *mut ObjectInner<T>)
            })
        } else {
            Err(self)
        }
    }

    pub fn get_member(self: &Rc<Self>, name: &str) -> Option<Object> {
        (self.object_vtable.get_member_fn)(self.get_object_row(), name)
            .or_else(|| self.object_type.as_ref()?.get_member(name).map(|m|
                m.call(tuple!(self.clone()))))
    }
}

impl<T: ObjectTrait> Deref for ObjectInner<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
