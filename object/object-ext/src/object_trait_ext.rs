use object_core::prelude::*;

use crate::object_ext::ObjectExt;

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

    fn from_data_type(self, object_type: Option<Object>) -> Object<Self> {
        ObjectInner::new(
            self,
            Self::OBJECT_VTABLE,
            object_type
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
        let input = input.on_matched(this.clone());
        if input.get_object_type()?.inner_type_id() == this.inner_type_id() {
            Some(input)
        } else {
            None
        }
    }
}
