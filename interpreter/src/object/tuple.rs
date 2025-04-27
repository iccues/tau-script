use crate::object_box::ObjectBox;

use super::{ObjectTrait, Object};

#[derive(Debug)]
pub struct Tuple {
    value: Vec<Object>,
}

impl Tuple {
    pub fn new(value: Vec<Object>) -> Object {
        ObjectBox::new(Tuple { value })
    }
    
    pub fn deconst(&self) -> &[Object] {
        &self.value
    }
}

impl ObjectTrait for Tuple {
    fn to_string_row(&self) -> String {
        let mut result = String::from("(");
        for (i, value) in self.value.iter().enumerate() {
            result.push_str(&value.to_string_row());
            if i != self.value.len() - 1 {
                result.push_str(", ");
            }
        }
        result.push_str(")");
        result
    }
}
