use std::ptr::drop_in_place;

use crate::types::error::error::Error;

use super::{obj_type::{ObjType, OBJ_TYPE_BOX}, object::Object};

pub trait ObjectTrait: Sized {
    const OBJ_TYPE: ObjType = ObjType {
        call: Self::call_fn,
        get_member: Self::get_member_fn,
        match_: Self::match_fn,
        on_matched: Self::on_matched_fn,
        drop: Self::drop_fn,
    };
    const OBJ_TYPE_TYPE: &Object = &OBJ_TYPE_BOX;

    const OBJ_TYPE_BOX: Object = Object::from_raw(&Self::OBJ_TYPE, Self::OBJ_TYPE_TYPE);

    fn get_member_fn(this: Object, name: &str) -> Object {
        let _ = (this, name);
        Error::new("get_member not implemented")
    }
    fn call_fn(this: Object, args: Object) -> Object {
        let _ = (this, args);
        Error::new("call not implemented")
    }
    fn match_fn(this: Object, other: Object) -> Option<Object> {
        let this_type = unsafe { this.get_data_uncheck::<ObjType>() };
        let other_matched = other.on_matched(this.clone());
        if this_type == other_matched.get_obj_type() {
            Some(other_matched.clone())
        } else {
            None
        }
    }
    fn on_matched_fn(this: Object, other: Object) -> Object {
        let _ = other;
        this
    }
    fn drop_fn(this: Object) {
        let this = this.get_data::<Self>().unwrap();
        unsafe {
            drop_in_place(this);
        }
    }

    fn from_data(data: Self) -> Object {
        Object::new(data, &Self::OBJ_TYPE_BOX)
    }
}
