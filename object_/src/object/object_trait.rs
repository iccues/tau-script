use std::{any::Any, fmt::Debug};

use crate::object::{object::{Object, ObjectInner}, object_vtable::ObjectVTable};

pub trait ObjectTrait: Any + Debug {}

pub trait ObjectTraitExt: ObjectTrait + Sized {
    const OBJECT_VTABLE: ObjectVTable<Self> = ObjectVTable {
        get_member_fn: Self::get_member,
        call_fn: if Self::CALLABLE {
            Some(Self::call)
        } else {
            None
        },
    };

    const CALLABLE: bool = false;

    fn get_object_type() -> Option<Object> {
        None
    }

    fn from_data(self) -> Object<Self> {
        ObjectInner::new(
            self,
            Self::OBJECT_VTABLE,
            Self::get_object_type(),
        )
    }

    fn get_member(&mut self, name: &str) -> Option<Object> {
        _ = name;
        None
    }

    fn call(&mut self, input: Object) -> Object {
        _ = input;
        unreachable!()
    }
}
