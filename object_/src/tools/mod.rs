use crate::{object::prelude::*, tuple};

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

#[macro_export]
macro_rules! matches_ {
    ( $front:tt $( : $type_:ty )? = $value:expr ) => {
        let $crate::matches_front!($front) = $crate::matches_back!($front $( : $type_ )?, $value);
    }
}

#[macro_export]
macro_rules! matches_front {
    ($id:ident $( : $type_:ty )? ) => {
        $id
    };
    (( $( $elements:tt $( : $type_:ty )? ),* $(,)? )) => {
        ( $( $crate::matches_front!( $elements $( : $type_ )? ) ),* , )
    };
}

#[macro_export]
macro_rules! matches_back {
    ($id:ident, $value:expr) => {
        $value
    };
    ($id:ident : $type_:ty, $value:expr) => {
        $crate::tools::match_downcast::<$type_>($value).unwrap()
    };
    (( $( $elements:tt $( : $type_:ty )? ),* $(,)? ), $value:expr) => {
        {
            let mut elements = $crate::tools::match_downcast::<$crate::types::tuple::Tuple>($value).unwrap().get_vec().into_iter();
            let tuple = (
                $(
                    $crate::matches_back!($elements  $( : $type_ )?, elements.next().unwrap()),
                )*
            );
            if elements.next().is_some() {
                panic!("Too many elements in tuple");
            }
            tuple
        }
    };
}
