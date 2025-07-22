use std::ops::{Deref, DerefMut};
use std::{mem::transmute, rc::Rc};

use crate::prelude::*;

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
        (self.clone() as Object).get_member(name)
    }
}

impl<T: ObjectTrait + ?Sized> ObjectInner<T> {
    pub fn call(self: &Rc<Self>, input: Object) -> Object {
        (self.object_vtable.call_fn.unwrap())(self.get_object_row(), input)
    }

    pub fn try_call(self: &Rc<Self>) -> Option<Box<dyn Fn(Object) -> Object>> {
        let self_ = self.clone();
        if let Some(call_fn) = self.object_vtable.call_fn {
            Some(Box::new(move |input| call_fn(self_.get_object_row(), input)))
        } else {
            None
        }
    }

    pub fn match_(self: &Rc<Self>, input: Object) -> Option<Object> {
        self.object_vtable.match_fn?(self.get_object_row(), input)
    }

    pub fn try_match(self: &Rc<Self>) -> Option<Box<dyn Fn(Object) -> Option<Object>>> {
        if let Some(match_fn) = self.object_vtable.match_fn {
            let self_ = self.clone();
            Some(Box::new(move |input| match_fn(self_.get_object_row(), input)))
        } else {
            None
        }
    }

    pub fn get_object_type(self: &Rc<Self>) -> Option<Object> {
        self.object_type.clone()
    }

    pub fn is<U: ObjectTrait>(&self) -> bool {
        self.object.type_id() == std::any::TypeId::of::<U>()
    }

    fn get_object_row(self: &Rc<Self>) -> Object<()> {
        unsafe { Rc::from_raw(Rc::into_raw(self.clone()) as *mut ObjectInner<()>) }
    }

    pub fn inner_type_id(&self) -> std::any::TypeId {
        self.object.type_id()
    }
}

impl ObjectInner<dyn ObjectTrait> {
    pub fn downcast<T: ObjectTrait>(self: &Rc<Self>) -> Option<Object<T>> {
        if self.is::<T>() {
            Some(unsafe {
                Rc::from_raw(Rc::into_raw(self.clone()) as *mut ObjectInner<T>)
            })
        } else {
            None
        }
    }

    pub fn get_member(self: &Rc<Self>, name: &str) -> Option<Object> {
        (self.object_vtable.get_member_fn)(self.get_object_row(), name)
            .or_else(|| self.object_type.as_ref()?.get_member(name).map(|m|
                m.call(self.clone())))
    }
}

impl<T: ObjectTrait> Deref for ObjectInner<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<T: ObjectTrait> DerefMut for ObjectInner<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}
