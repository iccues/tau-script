use crate::object::{object_trait::ObjectTrait, object::Object};

#[derive(PartialEq)]
pub struct ObjType {
    pub call: fn(Object, Object) -> Object,
    pub get_member: fn(Object, &str) -> Object,
    pub match_: fn(Object, Object) -> Option<Object>,
}

impl ObjectTrait for ObjType {}

impl ObjType {
    fn call_fn(_: Object, _: Object) -> Object {
        panic!("call not implemented")
    }

    fn get_member_fn(_: Object, _: &str) -> Object {
        panic!("get_member not implemented")
    }

    fn match_fn(this: Object, other: Object) -> Option<Object> {
        let this = unsafe { this.get_data_uncheck::<ObjType>() };
        if this == other.get_obj_type() {
            Some(other.clone())
        } else {
            None
        }
    }
}

static OBJ_TYPE: ObjType = ObjType {
    call: ObjType::call_fn,
    get_member: ObjType::get_member_fn,
    match_: ObjType::match_fn,
};

pub static OBJ_TYPE_BOX: Object = Object::from_raw(&OBJ_TYPE, &OBJ_TYPE_BOX);

#[cfg(test)]
mod tests {
    use crate::types::{bool::Bool, number::Integer};

    #[test]
    fn test_obj_type_match() {
        let obj1 = Integer::new(42);
        assert_eq!(obj1.get_data::<Integer>(), Some(&mut Integer { value: 42 }));
        assert_eq!(obj1.get_data::<Bool>(), None);
    }
}
