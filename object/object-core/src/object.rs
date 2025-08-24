use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::mem::transmute;

use crate::error::{ObjectError, ObjectResult};
use crate::prelude::*;

pub type Object<T = dyn ObjectTrait> = Arc<ObjectInner<T>>;

#[derive(Debug)]
pub struct ObjectInner<T: ObjectTrait + ?Sized = dyn ObjectTrait> {
    object_type: Option<Object>,
    object_vtable: ObjectVTable<()>,
    object: T,
}

impl<T: ObjectTrait> ObjectInner<T> {
    pub fn new(object: T, object_vtable: ObjectVTable<T>, object_type: Option<Object>) -> Object<T> {
        Arc::new(ObjectInner {
            object_type,
            object_vtable: unsafe { transmute(object_vtable) },
            object,
        })
    }

    pub fn get_member(self: &Arc<Self>, name: &str) -> ObjectResult<Object> {
        (self.clone() as Object).get_member(name)
    }

    pub fn try_call(self: &Arc<Self>) -> ObjectResult<Box<dyn Fn(Object) -> Object>> {
        (self.clone() as Object).try_call()
    }

    pub fn try_match(self: &Arc<Self>) -> ObjectResult<Box<dyn Fn(Object) -> ObjectResult<Object>>> {
        (self.clone() as Object).try_match()
    }
}

impl<T: ObjectTrait + ?Sized> ObjectInner<T> {
    pub fn call(self: &Arc<Self>, input: Object) -> Object {
        (self.object_vtable.call_fn.unwrap())(self.get_object_row(), input)
    }

    pub fn match_(self: &Arc<Self>, input: Object) -> Option<Object> {
        self.object_vtable.match_fn?(self.get_object_row(), input)
    }

    pub fn get_object_type(self: &Arc<Self>) -> Option<Object> {
        self.object_type.clone()
    }

    pub fn is<U: ObjectTrait>(&self) -> bool {
        self.object.type_id() == std::any::TypeId::of::<U>()
    }

    fn get_object_row(self: &Arc<Self>) -> Object<()> {
        unsafe { Arc::from_raw(Arc::into_raw(self.clone()) as *mut ObjectInner<()>) }
    }

    pub fn inner_type_id(&self) -> std::any::TypeId {
        self.object.type_id()
    }
}

impl ObjectInner<dyn ObjectTrait> {
    pub fn downcast<T: ObjectTrait>(self: &Arc<Self>) -> ObjectResult<Object<T>> {
        if self.is::<T>() {
            Ok(unsafe {
                Arc::from_raw(Arc::into_raw(self.clone()) as *mut ObjectInner<T>)
            })
        } else {
            Err(ObjectError::DowncastFailed(
                self.clone(),
                std::any::TypeId::of::<T>(),
            ))
        }
    }

    pub fn get_member(self: &Arc<Self>, name: &str) -> ObjectResult<Object> {
        if let Some(member) = (self.object_vtable.get_member_fn)(self.get_object_row(), name) {
            Ok(member)
        } else if let Some(object_type) = &self.object_type {
            object_type.get_member(name).map(|m| m.call(self.clone()))
        } else {
            Err(ObjectError::MemberNotFound(self.clone(), name.to_string()))
        }
    }

    pub fn try_call(self: &Arc<Self>) -> ObjectResult<Box<dyn Fn(Object) -> Object>> {
        let self_ = self.clone();
        if let Some(call_fn) = self.object_vtable.call_fn {
            Ok(Box::new(move |input| call_fn(self_.get_object_row(), input)))
        } else {
            Err(ObjectError::CallNotImplemented(self.clone()))
        }
    }

    pub fn try_match(self: &Arc<Self>) -> ObjectResult<Box<dyn Fn(Object) -> ObjectResult<Object>>> {
        let self_ = self.clone();
        if let Some(match_fn) = self.object_vtable.match_fn {
            Ok(Box::new(move |input| match_fn(self_.get_object_row(), input).ok_or(
                ObjectError::MatchNotImplemented(self_.clone()),
            )))
        } else {
            Err(ObjectError::MatchNotImplemented(self_))
        }
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

impl<T: ObjectTrait + ?Sized> Display for ObjectInner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object<{}>", std::any::type_name_of_val(&self.object))
    }
}

// unsafe impl<T: ObjectTrait + ?Sized> Sync for ObjectInner<T> {}
// unsafe impl<T: ObjectTrait + ?Sized> Send for ObjectInner<T> {}
