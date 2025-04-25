use crate::{object::tuple::Tuple, object_box::ObjectBox};

use super::{closure::Closure, object_fn::ObjectTraitFn, Object, ObjectTrait};

#[derive(Debug)]
pub struct Integer {
    value: i64,
}

impl Integer {
    pub fn new(literal: &str) -> Object {
        let value = literal.parse::<i64>().unwrap();
        ObjectBox::new(Integer { value })
    }

    pub fn add(input: Object) -> Object {
        match input.downcast::<Tuple>().unwrap().deconst() {
            [a, b] => {
                let a = a.clone().downcast::<Integer>().unwrap();
                let b = b.clone().downcast::<Integer>().unwrap();
                let result = a.value + b.value;
                ObjectBox::new(Integer { value: result })
            }
            _ => panic!("Invalid input"),
        }
    }

    pub fn add_curry(&self) -> Object {
        Closure::new(ObjectBox::new(
            Integer::add as ObjectTraitFn),
            vec![Some(ObjectBox::new(Self { value: self.value }))]
        )
    }
}

impl ObjectTrait for Integer {
    fn to_string_row(&self) -> String {
        self.value.to_string()
    }

    fn get_member(&self, name: &str) -> super::Object {
        match name {
            "add" => self.add_curry(),
            _ => panic!("Member not found"),
        }
    }
}
