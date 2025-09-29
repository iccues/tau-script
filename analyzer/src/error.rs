use std::rc::Rc;

pub type Result<T> = std::result::Result<T, AnalyzerError>;

#[derive(Debug, thiserror::Error, Clone)]
pub enum AnalyzerError {
    #[error("IO error: {0}")]
    Io(Rc<std::io::Error>),

    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Parse float error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("")]
    None,

    #[error("Failed to downcast token to type {0}")]
    DowncastFailed(&'static str),

    #[error("Unknown token encountered")]
    UnknownToken,
}

impl From<std::io::Error> for AnalyzerError {
    fn from(err: std::io::Error) -> Self {
        AnalyzerError::Io(Rc::new(err))
    }
}

impl AnalyzerError {
    pub fn is_fatal(&self) -> bool {
        matches!(self, AnalyzerError::Io(_) | AnalyzerError::UnknownToken)
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
