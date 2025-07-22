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
        ( $( $crate::matches_front!( $elements $( : $type_ )? ) , )* )
    };
}

#[macro_export]
macro_rules! matches_back {
    ($id:ident, $value:expr) => {
        $value
    };
    ($id:ident : $type_:ty, $value:expr) => {
        {
            use $crate::object_ext::ObjectExt;
            $value.match_downcast::<$type_>().unwrap()
        }
    };
    (( $( $elements:tt $( : $type_:ty )? ),* $(,)? ), $value:expr) => {
        {
            use $crate::object_ext::ObjectExt;
            let mut elements = $value.match_downcast::<$crate::tuple::Tuple>().unwrap().get_vec().into_iter();
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
