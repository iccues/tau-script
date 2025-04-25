use crate::object_box::ObjectBox;

use super::{Object, ObjectTrait};

#[derive(Debug)]
pub struct String_ {
    value: String,
}

impl String_ {
    pub fn new(literal: &str) -> Object {
        let value = literal[1..literal.len() - 1].to_string();
        ObjectBox::new(String_ { value })
    }

    pub fn to_string_row(&self) -> String {
        self.value.clone()
    }
}

impl ObjectTrait for String_ {
    fn to_string_row(&self) -> String {
        self.value.clone()
    }

    fn get_member(&self, name: &str) -> super::Object {
        match name {
            _ => panic!("Member not found"),
        }
    }
}
