use std::rc::Rc;

use crate::object_box::ObjectBox;

use super::{closure::Closure, object_fn::ObjectTraitFn, tuple::Tuple, undefined::Undefined, Object, ObjectTrait};

#[derive(Debug)]
pub struct Variable {
    value: Option<Object>,
}

impl Variable {
    pub fn new() -> Object {
        ObjectBox::new(Variable { value: None })
    }

    pub fn get(input: Object) -> Object {
        match input.downcast::<Tuple>().unwrap().deconst() {
            [a] => {
                a.clone()
            }
            _ => panic!("Invalid input"),
        }
    }

    pub fn set(input: Object) -> Object {
        match input.downcast::<Tuple>().unwrap().deconst() {
            [a, b] => {
                a.clone().downcast_unget::<Variable>().unwrap().value = Some(b.clone());
                a.clone()
            }
            _ => panic!("Invalid input"),
        }
    }
}

impl ObjectTrait for Variable {
    fn to_string_row(&self) -> String {
        match &self.value {
            Some(value) => value.to_string_row(),
            None => "None".to_string(),
        }
    }

    fn get_member(self: Rc<Self>, name: &str) -> Object {
        if name == "set" {
            Closure::new(
                ObjectBox::new(Variable::set as ObjectTraitFn),
                vec![Some(ObjectBox::new_rc(self.clone()))]
            )
        } else if name == "get" {
            Closure::new(
                ObjectBox::new(Variable::get as ObjectTraitFn),
                vec![Some(self.value.clone().unwrap_or(Undefined::new()))]
            )
        } else {
            self.value.clone().unwrap_or(Undefined::new()).get_member(name)
        }
    }
}
