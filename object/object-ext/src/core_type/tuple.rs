use object_core::prelude::*;

use crate::object_trait_ext::ObjectTraitExt;

#[derive(Debug, ObjectTrait)]
pub struct Tuple {
    elements: Vec<Object>,
}

impl ObjectTraitExt for Tuple {}

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
        $crate::core_type::tuple::Tuple::new(vec![$($item)*])
    };
}
