use std::{ops::{Deref, DerefMut}, rc::Rc};

use crate::object::{undefined::Undefined, ObjectTrait};

#[derive(Debug)]
pub struct ObjectBox<T: 'static + ?Sized> {
    inner: Rc<T>
}

impl ObjectBox<dyn ObjectTrait> {
    pub fn new<T: 'static + ObjectTrait>(inner: T) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }

    pub fn new_rc<T: 'static + ObjectTrait>(inner: Rc<T>) -> Self {
        Self { inner }
    }

    pub fn downcast<U: ObjectTrait>(self) -> Result<ObjectBox<U>, Self> {
        if !self.inner.clone().get_member("get").is::<Undefined>() {
            return self.inner.clone().get_member("get").downcast::<U>();
        }

        match self.inner.downcast_rc::<U>() {
            Ok(rc) => Ok(ObjectBox { inner: rc }),
            Err(rc) => Err(ObjectBox { inner: rc }),
        }
    }

    pub fn downcast_unget<U: ObjectTrait>(self) -> Result<ObjectBox<U>, Self> {
        match self.inner.downcast_rc::<U>() {
            Ok(rc) => Ok(ObjectBox { inner: rc }),
            Err(rc) => Err(ObjectBox { inner: rc }),
        }
    }

    pub fn get_member(&self, name: &str) -> ObjectBox<dyn ObjectTrait> {
        self.inner.clone().get_member(name)
    }
}

impl<T: ?Sized> Clone for ObjectBox<T> {
    fn clone(&self) -> Self {
        ObjectBox {
            inner: self.inner.clone(),
        }
    }
}

impl<T: ?Sized> Deref for ObjectBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: ?Sized> DerefMut for ObjectBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let r = self.inner.deref() as *const T as *mut T;

        #[allow(invalid_reference_casting)]
        unsafe {
            &mut *r
        }
    }
}


// impl<T: ?Sized> std::ops::Receiver for ObjectBox<T> {
    
// }
// impl Rece for  {
    
// }