use crate::object::{object_trait::ObjectTrait, object::Object};

#[derive(PartialEq)]
pub struct ObjType {
    pub call: fn(Object, Object) -> Object,
    pub get_member: fn(Object, &str) -> Object,
    pub match_: fn(Object, Object) -> Option<Object>,
    pub on_matched: fn(Object, Object) -> Object,
    pub drop: fn(Object),
}

impl ObjectTrait for ObjType {}

impl ObjType {}

static OBJ_TYPE: ObjType = ObjType {
    call: ObjType::call_fn,
    get_member: ObjType::get_member_fn,
    match_: ObjType::match_fn,
    on_matched: ObjType::on_matched_fn,
    drop: ObjType::drop_fn,
};

pub static OBJ_TYPE_BOX: Object = Object::from_raw(&OBJ_TYPE, &OBJ_TYPE_BOX);

#[cfg(test)]
mod tests {
    use crate::types::{bool::Bool, number::Integer};

    #[test]
    fn test_obj_type_match() {
        let obj1 = Integer::new(42);
        assert_eq!(obj1.get_data_match::<Integer>(), Some(&mut Integer { value: 42 }));
        assert_eq!(obj1.get_data_match::<Bool>(), None);
    }
}
