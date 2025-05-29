use crate::object::{object::Object, object_trait::ObjectTrait};

use crate::types::callable::closure::Closure;
use crate::types::compound::tuple::Tuple;
use crate::types::control::undefined::Undefined;
use crate::types::error::error::Error;

pub struct Variable {
    pub value: Object,
}

impl ObjectTrait for Variable {
    fn get_member_fn(this: Object, name: &str) -> Object {
        match name {
            "set" => Closure::new_first(Variable::set, this),
            name => {
                let this = this.get_data::<Variable>().unwrap();
                this.value.get_member(name)
            }
        }
    }

    fn on_matched_fn(this: Object, other: Object) -> Object {
        let this_type = this.get_data::<Variable>().unwrap();
        if other.get_data::<Variable>().is_some() {
            this
        } else {
            this_type.value.on_matched(other)
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
        let Some(tuple) = input.get_data_match::<Tuple>() else {
            return Error::new("Invalid input");
        };
        let [a, b] = tuple.as_slice() else {
            return Error::new("Invalid input");
        };
        let Some(var) = a.get_data::<Variable>() else {
            return Error::new("First element must be a Variable");
        };
        var.value = b.clone();
        a.clone()
    }
}
