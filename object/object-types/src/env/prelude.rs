use std::sync::LazyLock;

use object_core::prelude::*;
use object_ext::module;
use object_ext::object_trait_ext::ObjectTraitExt;


#[allow(non_upper_case_globals)]
pub static prelude: LazyLock<Object> = module! {
    string = object_ext::core_type::string::ObjStringType.from_data();
    int = crate::primitive::numbers::ObjI64Type.from_data();
    list_type = crate::list::ObjListTypeType.from_data();
    any = crate::unit::any::ObjAny.from_data();
};
