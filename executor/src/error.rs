pub type ExecutorResult<T> = Result<T, Executor>;

#[derive(Debug, thiserror::Error)]
pub enum Executor {
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("{0}")]
    ObjectError(#[from] object::core::error::ObjectError),

    #[error("{0}")]
    FrontendError(#[from] frontend::frontend_library::error::FrontendError),
}
