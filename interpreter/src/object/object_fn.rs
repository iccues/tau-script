use std::fmt::Debug;

use crate::object_box::ObjectBox;

use super::{ObjectTrait, Object};

pub type ObjectTraitFn = fn(Object) -> Object;
pub type ObjectTraitFnBox = Box<ObjectTraitFn>;

impl ObjectTrait for ObjectTraitFn {
    fn to_string_row(&self) -> String {
        "function".to_string()
    }

    fn call(&self, input: Object) -> Object {
        self(input)
    }
}

pub fn print_func(input: Object) -> Object {
    println!("{}", input.to_string_row());
    input
}



pub struct ObjectClosure {
    func: Box<dyn Fn(Object) -> Object>,
}

impl ObjectClosure {
    pub fn new(func: impl Fn(Object) -> Object + 'static) -> Object {
        ObjectBox::new(ObjectClosure { func: Box::new(func) })
    }
}

impl ObjectTrait for ObjectClosure {
    fn to_string_row(&self) -> String {
        "ObjectClosure".to_string()
    }

    fn call(&self, input: Object) -> Object {
        (self.func)(input)
    }
}

impl Debug for ObjectClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ObjectClosure")
    }
}
