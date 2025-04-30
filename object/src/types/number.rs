use crate::object::{object::Object, object_trait::ObjectTrait};

use super::{closure::Closure, func::Func, tuple::Tuple};

#[derive(Clone)]
pub struct Integer {
    pub value: i32,
}

impl ObjectTrait for Integer {
    fn get_member_fn(this: Object, name: &str) -> Object {
        match name {
            "add" => this.get_data_as::<Integer>().add_curry(),
            _ => panic!("get_member not implemented"),
        }
    }
}

impl Integer {
    pub fn new(value: i32) -> Object {
        Self::from_data(Integer { value })
    }


    fn add(input: Object) -> Object {
        match input.get_data_as::<Tuple>().elements.as_slice() {
            [a, b] => {
                let a = a.get_data_as::<Integer>();
                let b = b.get_data_as::<Integer>();
                let result = a.value + b.value;
                Integer::new(result)
            }
            _ => panic!("Invalid input"),
        }
    }

    fn add_curry(&self) -> Object {
        Closure::new(
            Func::new(Integer::add),
            vec![Some(Self::from_data(self.clone()))],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let int1 = Integer::new(42);
        let int2 = Integer::new(1);

        let sum = int1.get_member_("add").call_(Tuple::new(vec![int2]));

        assert_eq!(
            sum.get_data_as::<Integer>().value,
            43
        );
    }
}
