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

    pub fn try_get<const N: usize>(&self) -> Option<[Object; N]> {
        self.elements.clone().try_into().ok()
    }

    pub fn get_vec(&self) -> Vec<Object> {
        self.elements.clone()
    }
}

#[macro_export]
macro_rules! tuple {
    ($($item:tt)*) => {
        $crate::types::tuple::Tuple::new(vec![$($item)*])
    };
}
