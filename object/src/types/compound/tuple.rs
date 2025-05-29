use crate::object::{object::Object, object_trait::ObjectTrait};

use crate::types::error::error::Error;
use crate::types::primitive::number::Integer;

pub struct Tuple {
    elements: Vec<Object>,
}

impl ObjectTrait for Tuple {
    fn get_member_fn(this: Object, name: &str) -> Object {
        match name {
            "len" => Tuple::len(this),
            _ => Error::new("get_member not implemented"),
            
        }
    }
}

impl Tuple {
    pub fn new(elements: Vec<Object>) -> Object {
        Self::from_data(Tuple { elements })
    }

    pub fn len(this: Object) -> Object {
        let tuple: &Tuple = this.get_data::<Tuple>().unwrap();
        Integer::new(tuple.elements.len() as i32)
    }

    pub fn as_slice(&self) -> &[Object] {
        &self.elements
    }
}

#[macro_export]
macro_rules! tuple {
    ($($item:tt)*) => {
        $crate::types::compound::tuple::Tuple::new(vec![$($item)*])
    };
}

#[macro_export]
macro_rules! tuple_match {
    ($input:expr, ($($name:ident : $ty:ty),* $(,)? ) $then:block) => {
        tuple_match!($input, ($($name : $ty),*) $then else {
            $crate::types::error::error::Error::new("Tuple match failed")
        })
    };
    ($input:expr, ($($name:ident : $ty:ty),* $(,)? ) $then:block else $else:block) => { 'tuple_match: {
        let Some(tuple) = $input.get_data_match::<$crate::types::compound::tuple::Tuple>() else {
            break 'tuple_match ($else);
        };
        let [$($name),+] = tuple.as_slice() else {
            break 'tuple_match ($else);
        };
        $(
            let Some($name) = $name.get_data_match::<$ty>() else {
                break 'tuple_match ($else);
            };
        )*
        $then
    }};
}

#[cfg(test)]
mod tests {

    use crate::types::primitive::number::Integer;
    use crate::types::compound::tuple::Tuple;

    #[test]
    fn test_tuple() {
        let tuple = Tuple::new(vec![Integer::new(1), Integer::new(2), Integer::new(3)]);

        let len = tuple.get_member("len");
        assert_eq!(len.get_data::<Integer>().unwrap().value, 3);
    }
}
