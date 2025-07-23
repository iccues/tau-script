use std::rc::Rc;

pub type FrontendResult<T> = Result<T, FrontendError>;

#[derive(Debug, thiserror::Error, Clone)]
pub enum FrontendError {
    #[error("IO error: {0}")]
    Io(#[from] Rc<std::io::Error>),

    #[error("")]
    None,

    #[error("")]
    DowncastFailed,

    #[error("Unknown token encountered")]
    UnknownToken,
}

impl FrontendError {
    pub fn is_fatal(&self) -> bool {
        matches!(self, FrontendError::Io(_) | FrontendError::UnknownToken)
    }
    // pub fn is_recoverable(&self) -> bool {
    //     matches!(self, FrontendError::Io(_) | FrontendError::None)
    // }
}

#[macro_export]
macro_rules! try_parse {
    ($expr:expr) => {
        match $expr {
            Ok(val) => return Ok(val),
            Err(err) if err.is_fatal() => return Err(err),
            Err(_) => {},
        }
    };
}
