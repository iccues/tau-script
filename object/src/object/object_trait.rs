use std::{any::Any, fmt::Debug};

use crate::{object::{object::{Object, ObjectInner}, object_vtable::ObjectVTable}, tools::on_matched};

pub trait ObjectTrait: Any + Debug {}

impl ObjectTrait for () {}

pub trait ObjectTraitExt: ObjectTrait + Sized {
    const OBJECT_VTABLE: ObjectVTable<Self> = ObjectVTable {
        get_member_fn: Self::get_member,
        call_fn: if Self::CALLABLE {
            Some(Self::call)
        } else {
            None
        },
        match_fn: if Self::MATCHABLE {
            Some(Self::match_)
        } else {
            None
        },
    };

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

    fn get_member(_this: Object<Self>, name: &str) -> Option<Object> {
        _ = name;
        None
    }

    const CALLABLE: bool = false;
    fn call(this: Object<Self>, input: Object) -> Object {
        _ = this;
        _ = input;
        unreachable!()
    }

    const MATCHABLE: bool = false;
    fn match_(this: Object<Self>, input: Object) -> Option<Object> {
        let input = on_matched(input, this.clone());
        if input.get_object_type()?.inner_type_id() == this.inner_type_id() {
            Some(input)
        } else {
            None
        }
    }
}
