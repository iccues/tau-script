use object_core::prelude::{Object, ObjectTrait};

use crate::{object_trait_ext::ObjectTraitExt, tuple};

pub trait ObjectExt : Sized {
    fn match_downcast<T: ObjectTraitExt>(&self) -> Option<Object<T>>;
    fn on_matched(&self, model: Object) -> Object;
}

impl ObjectExt for Object {
    fn match_downcast<T: ObjectTraitExt>(&self) -> Option<Object<T>> {
        let self_;
        if let Some(model) = T::get_object_type() {
            if let Some(match_fn) = model.try_match() {
                self_ = match_fn(self.clone())?;
            } else {
                self_ = self.on_matched(model);
            }
        } else {
            self_ = self.clone();
        }
        self_.downcast::<T>()
    }

    fn on_matched(&self, model: Object) -> Object {
        if let Some(on_matched_fn) = self.get_member("on_matched") {
            on_matched_fn.call(tuple!(model.clone()))
        } else {
            self.clone()
        }
    }
}

impl<U: ObjectTrait> ObjectExt for Object<U> {
    fn match_downcast<T: ObjectTraitExt>(&self) -> Option<Object<T>> {
        (self.clone() as Object).match_downcast()
    }

    fn on_matched(&self, model: Object) -> Object {
        (self.clone() as Object).on_matched(model)
    }
}
