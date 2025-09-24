use std::fmt;

#[derive(Debug)]
pub enum PresentationError {
    NotFound(String),
}

impl fmt::Display for PresentationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PresentationError::NotFound(arg) => write!(f, "Entity not found {}", arg),
        }
    }
}

impl std::error::Error for PresentationError {}
