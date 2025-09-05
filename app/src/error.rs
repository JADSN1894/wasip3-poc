use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Wastime(wasmtime::Error),
    Unit(()),
    #[allow(dead_code)]
    Custom(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wastime(inner) => write!(f, "{inner}"),
            Self::Unit(inner) => write!(f, "{inner:?}"),
            Self::Custom(inner) => write!(f, "{inner}"),
        }
    }
}

impl From<wasmtime::Error> for AppError {
    fn from(value: wasmtime::Error) -> Self {
        Self::Wastime(value)
    }
}

impl From<()> for AppError {
    fn from(value: ()) -> Self {
        Self::Unit(value)
    }
}
