use crate::object::prelude::*;

#[derive(Debug)]
pub struct Tuple {
    elements: Vec<Object>,
}

impl ObjectTrait for Tuple {}

impl ObjectTraitExt for Tuple {
    
}

impl Tuple {
    pub fn new(elements: Vec<Object>) -> Object {
        Self::from_data(Tuple { elements })
    }

    pub fn as_slice(&self) -> &[Object] {
        &self.elements
    }
}

#[macro_export]
macro_rules! tuple {
    ($($item:tt)*) => {
        $crate::types::tuple::Tuple::new(vec![$($item)*])
    };
}
