mod object;
mod object_trait;
mod object_vtable;
pub mod error;

pub mod prelude {
    pub use crate::object::{Object, ObjectInner};
    pub use crate::object_trait::ObjectTrait;
    pub use crate::object_vtable::ObjectVTable;
}
