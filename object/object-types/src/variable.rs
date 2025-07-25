use std::cell::RefCell;

use object_core::prelude::*;
use object_ext::object_ext::ObjectExt;
use object_ext::object_trait_ext::ObjectTraitExt;
use object_ext::{matches_, tuple};
use crate::{callable::{closure::Closure, rust_func::RustFunc}, unit::undefined::Undefined};

#[derive(Debug)]
struct VariableType;

impl ObjectTrait for VariableType {}

impl ObjectTraitExt for VariableType {
    fn get_member(_this: Object<Self>, name: &str) -> Option<Object> {
        match name {
            "set" => Some(Closure::new(RustFunc::new(Variable::set), 2)),
            "on_matched" => Some(Closure::new(RustFunc::new(Variable::on_matched), 2)),
            _ => None,
        }
    }

    const MATCHABLE: bool = true;
}


#[derive(Debug)]
pub struct Variable {
    value: RefCell<Object>
}

impl ObjectTrait for Variable {}

impl ObjectTraitExt for Variable {
    fn get_member(this: Object<Self>, name: &str) -> Option<Object> {
        match name {
            "set" | "on_matched" => None,
            name => this.value.borrow().get_member(name).ok(),
        }
    }

    fn get_object_type() -> Option<Object> {
        Some(VariableType.from_data())
    }

    const CALLABLE: bool = true;
    fn call(this: Object<Self>, input: Object) -> Object {
        this.value.borrow().call(input)
    }

    const MATCHABLE: bool = true;
    fn match_(this: Object<Self>, input: Object) -> Option<Object> {
        this.value.borrow().match_(input)
    }
}

impl Variable {
    pub fn new() -> Object<Variable> {
        Variable {
            value: RefCell::new(Undefined::new()),
        }.from_data()
    }

    pub fn from(value: Object) -> Object<Variable> {
        Variable {
            value: RefCell::new(value),
        }.from_data()
    }

    fn set(input: Object) -> Object {
        matches_!((a: Variable, (b)) = input);

        *a.value.borrow_mut() = b.clone();

        tuple!()
    }

    fn on_matched(input: Object) -> Object {
        matches_!((a, (b)) = input);
        let a: Object<Variable> = a.downcast().unwrap();

        if b.is::<VariableType>() {
            a
        } else {
            a.value.borrow().clone().on_matched(b.clone())
        }
    }
}


#[cfg(test)]
mod tests {
    // use object_ext::tools::match_downcast;
    use object_ext::object_ext::ObjectExt;
    use crate::primitive::numbers::ObjI64;
    use super::*;

    #[test]
    fn test_variable_set() {
        let a = Variable::new();
        let b = ObjI64::new(42);
        a.get_member("set").unwrap().call(tuple!(b));
        assert_eq!(a.value.borrow().downcast::<ObjI64>().unwrap().value, 42);
    }

    #[test]
    fn test_variable_on_matched() {
        let a = Variable::from(ObjI64::new(42));
        let b: Object<ObjI64> = a.match_downcast().unwrap();
        assert_eq!(b.value, 42);
    }

    #[test]
    fn test_variable_on_matched2() {
        let a: Object = ObjI64::new(20);
        let b: Object = Variable::from(ObjI64::new(22));
        let c = a.get_member("add").unwrap().call(tuple!(b));
        assert_eq!(c.downcast::<ObjI64>().unwrap().value, 42)
    }

    #[test]
    fn test_variable_get_member() {
        let a = Variable::from(ObjI64::new(20));
        let b = ObjI64::new(22);
        let c = a.get_member("add").unwrap().call(tuple!(b));
        assert_eq!(c.downcast::<ObjI64>().unwrap().value, 42);
    }
}
