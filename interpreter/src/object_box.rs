use std::{ops::{Deref, DerefMut}, rc::Rc};

use crate::object::ObjectTrait;

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

    pub fn downcast<U: ObjectTrait>(self) -> Result<ObjectBox<U>, Self> {
        match self.inner.downcast_rc::<U>() {
            Ok(rc) => Ok(ObjectBox { inner: rc }),
            Err(rc) => Err(ObjectBox { inner: rc }),
        }
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
