use crate::object::prelude::*;
use crate::types::{callable::{closure::Closure, rust_func::RustFunc}, tuple::Tuple};

#[derive(Debug)]
pub struct IntegerType;

impl ObjectTrait for IntegerType {}

impl ObjectTraitExt for IntegerType {
    fn get_member(&mut self, name: &str) -> Option<Object> {
        match name {
            "add" => Some(Closure::new(RustFunc::new(Integer::add), 2)),
            _ => None
        }
    }
}


#[derive(Debug)]
pub struct Integer {
    value: i64,
}

impl ObjectTrait for Integer {}

impl ObjectTraitExt for Integer {
    fn get_object_type() -> Option<Object> {
        Some(IntegerType.from_data())
    }

    const CALLABLE: bool = true;
    fn call(&mut self, input: Object) -> Object {
        println!("calling");
        Integer { value: self.value + input.downcast::<Integer>().unwrap().value }.from_data()
    }
}

impl Drop for Integer {
    fn drop(&mut self) {
        println!("{:p} is droped", self);
    }
}

impl Integer {
    pub fn new(value: i64) -> Object<Integer> {
        Integer { value }.from_data()
    }

    fn add(input: Object) -> Object {
        let input = input.downcast::<Tuple>().unwrap();
        let [a, b] = input.as_slice() else {
            panic!()
        };
        dbg!(a);
        let a = a.clone().downcast::<Tuple>().unwrap();
        let b = b.clone().downcast::<Tuple>().unwrap();
        let [a] = a.as_slice() else {
            panic!()
        };
        let [b] = b.as_slice() else {
            panic!()
        };
        let a = a.clone().downcast::<Integer>().unwrap();
        let b = b.clone().downcast::<Integer>().unwrap();

        Integer::new(a.value + b.value)
        // unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple;

    use super::*;

    #[test]
    fn a() {
        let a: Object = Integer::new(20);
        let b: Object = Integer::new(22);
        assert_eq!(a.call(b).downcast::<Integer>().unwrap().value, 42);
    }

    #[test]
    fn b() {
        let a: Object = Integer::new(20);
        let b: Object = Integer::new(22);
        dbg!(a.get_member("add").unwrap().call(tuple!(b)));
    }
}
