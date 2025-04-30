use crate::object::{obj_type::{ObjType, OBJ_TYPE_BOX}, object::Object, object_trait::ObjectTrait};

use super::func::Func;

pub struct Integer {
    value: i32,
}

impl ObjectTrait for Integer {}

impl Integer {
    pub fn new(value: i32) -> Object {
        Object::new(
            Integer { value },
            &INTEGER_TYPE_BOX
        )
    }

    fn add(input: Object) -> Object {
        let integer = input.get_data_as::<Integer>();
        let result = integer.value + 1; // Example operation
        Integer::new(result)
    }

    fn call_fn(_: Object, _: Object) -> Object {
        panic!("call not implemented")
    }

    fn get_member_fn(_: Object, name: &str) -> Object {
        match name {
            "add" => Func::new(Integer::add),
            _ => panic!("get_member not implemented"),
        }
    }
}

// pub static INTEGER_TYPE_BOX: LazyLock<Object> = LazyLock::new(|| ObjType {
//     obj_head: ObjHead {
//         obj_type: OBJ_TYPE_BOX.clone(),
//         ref_count: 0,
//     },
//     call: Integer::call_fn,
//     get_member: Integer::get_member_fn,
// }.into_object_box());

pub static INTEGER_TYPE: ObjType = ObjType {
    call: Integer::call_fn,
    get_member: Integer::get_member_fn,
};

pub static INTEGER_TYPE_BOX: Object = Object::from_raw(&INTEGER_TYPE, &OBJ_TYPE_BOX);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let integer = Integer::new(42);
        integer.get_member_("add");
    }
}
