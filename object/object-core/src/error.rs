use thiserror::Error;

use crate::object::Object;

pub type ObjectResult<T> = Result<T, ObjectError>;

#[derive(Error, Debug)]
pub enum ObjectError {
    #[error("Member '{1}' not found in object '{0}'")]
    MemberNotFound(Object, String),

    #[error("Object '{0}' does not implement call")]
    CallNotImplemented(Object),

    #[error("Object '{0}' does not implement match")]
    MatchNotImplemented(Object),

    #[error("")]
    MatchFailed(Object),

    #[error("Downcast failed for object '{0:}' to type '{1:?}'")]
    DowncastFailed(Object, std::any::TypeId),
}
