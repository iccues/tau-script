use std::rc::Rc;

pub type FrontendResult<T> = Result<T, FrontendError>;

#[derive(Debug, thiserror::Error, Clone)]
pub enum FrontendError {
    #[error("IO error: {0}")]
    Io(Rc<std::io::Error>),

    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Parse float error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("")]
    None,

    #[error("")]
    DowncastFailed,

    #[error("Unknown token encountered")]
    UnknownToken,
}

impl From<std::io::Error> for FrontendError {
    fn from(err: std::io::Error) -> Self {
        FrontendError::Io(Rc::new(err))
    }
}

impl FrontendError {
    pub fn is_fatal(&self) -> bool {
        matches!(self, FrontendError::Io(_) | FrontendError::UnknownToken)
    }
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
