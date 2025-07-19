use crate::{object::prelude::{Object, ObjectTraitExt}, tuple};

pub fn on_matched(value: Object, model: Object) -> Object {
    if let Some(on_matched_fn) = value.get_member("on_matched") {
        on_matched_fn.call(tuple!(model.clone()))
    } else {
        value.clone()
    }
}

pub fn match_downcast<T: ObjectTraitExt>(mut value: Object) -> Option<Object<T>> {
    if let Some(model) = T::get_object_type() {
        dbg!(&model, &value);
        if let Some(match_fn) = model.try_match() {
            value = match_fn(value)?;
        } else {
            value = on_matched(value, model);
        }
    }
    value.downcast::<T>()
}


// macro_rules! matches {
//     () => {
        
//     };
// }
