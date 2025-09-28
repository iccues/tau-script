pub type MatchResult<T> = Result<T, MatchError>;

#[derive(Debug)]
pub enum MatchError {
    DowncastFailed {
        expected: &'static str,
        actual: String,
    },
    NotEnoughElements,
    TooManyElements,
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
            let mut elements = $value.match_downcast::<$crate::core_type::tuple::Tuple>().unwrap().get_vec().into_iter();
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


#[macro_export]
macro_rules! match_as {
    (( $( $types:tt ),* $(,)? ), $value:expr) => {
        (|| {
            let tuple = $crate::match_as!($crate::core_type::tuple::Tuple, $value)?;
            let mut elements = tuple.get_vec().into_iter();

            let result = (
                $(
                    {
                        let element = elements.next()
                            .ok_or($crate::macros::MatchError::NotEnoughElements)?;
                        $crate::match_as!($types, element)?
                    },
                )*
            );

            if elements.next().is_some() {
                return Err($crate::macros::MatchError::TooManyElements);
            }
            Ok(result)
        })()
    };
    ($type_:ty, $value:expr) => {
        {
            use $crate::object_ext::ObjectExt;
            $value.match_downcast::<$type_>()
                .map_err(|_| $crate::macros::MatchError::DowncastFailed {
                    expected: stringify!($type_),
                    actual: $value.to_string(),
                })
        }
    };
}
