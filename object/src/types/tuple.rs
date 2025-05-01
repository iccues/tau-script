use crate::object::{object::Object, object_trait::ObjectTrait};

use super::number::Integer;

pub struct Tuple {
    pub elements: Vec<Object>,
}

impl ObjectTrait for Tuple {
    fn get_member_fn(this: Object, name: &str) -> Object {
        match name {
            "len" => Tuple::len(this),
            _ => panic!("get_member not implemented"),
            
        }
    }
}

impl Tuple {
    pub fn new(elements: Vec<Object>) -> Object {
        Self::from_data(Tuple { elements })
    }

    pub fn len(this: Object) -> Object {
        let tuple: &Tuple = this.get_data::<Tuple>().unwrap();
        Integer::new(tuple.elements.len() as i32)
    }

    pub fn get(&self, index: usize) -> Option<&Object> {
        self.elements.get(index)
    }
}

#[cfg(test)]
mod tests {

    use crate::types::number::Integer;
    use crate::types::tuple::Tuple;

    #[test]
    fn test_tuple() {
        let tuple = Tuple::new(vec![Integer::new(1), Integer::new(2), Integer::new(3)]);

        let len = tuple.get_member("len");
        assert_eq!(len.get_data::<Integer>().unwrap().value, 3);
    }
}
