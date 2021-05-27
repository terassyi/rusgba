use std::fmt;

#[derive(Debug)]
pub enum GBAError {
    NotFound,
    InvalidData,

    // cpu
    InstructionNotFound,
}

pub type GBAResult<T> = Result<T, GBAError>;

impl fmt::Display for GBAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GBAError::*;
        match self {
            NotFound => write!(f, "Not Found."),
            InvalidData => write!(f, "Invalid Data."),
            InstructionNotFound => write!(f, "Instruction not found."),
        }
    }
}

impl std::error::Error for GBAError {}
