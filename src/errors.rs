use std::fmt;

#[derive(Debug)]
pub enum TaskError {
    CriticalError(String),
    GeneralError(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TaskError::CriticalError(ref msg) => write!(f, "Critical Error: {}", msg),
            TaskError::GeneralError(ref msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for TaskError {}
