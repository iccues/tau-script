mod object;
mod object_trait;
mod object_vtable;

pub mod prelude {
    pub use super::object::{Object, ObjectInner};
    pub use super::object_trait::{ObjectTrait, ObjectTraitExt};
    pub use super::object_vtable::ObjectVTable;
}
