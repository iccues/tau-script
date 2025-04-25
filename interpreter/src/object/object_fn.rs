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
