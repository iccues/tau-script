use crate::object::{object::Object, object_trait::ObjectTrait};

use super::{closure::Closure, func::Func, tuple::Tuple, undefined::Undefined};

pub struct Variable {
    pub value: Object,
}

impl ObjectTrait for Variable {
    fn get_member_fn(this: Object, name: &str) -> Object {
        match name {
            "set" => Closure::new(
                Func::new(Variable::set),
                vec![Some(this.clone())]
            ),
            name => {
                let this = this.get_data::<Variable>().unwrap();
                this.value.get_member(name)
            }
        }
    }
}

impl Variable {
    pub fn new() -> Object {
        Self::from_data(Variable {
            value: Undefined::new(),
        })
    }

    fn set(input: Object) -> Object {
        match input.get_data::<Tuple>().unwrap().elements.as_slice() {
            [a, b] => {
                a.get_data::<Variable>().unwrap().value = b.clone();
                a.clone()
            }
            _ => panic!("Invalid input"),
        }
    }
}
