use crate::types::GroupType;
use core::fmt;

/// Radio Data System Decoding Error
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    // #[error("Invalid input for field \"{field}\": `{value}`")]
    InvalidInput { field: &'static str, value: u16 },
    // #[error("Unimplemented RDS Group Type: {:?}", 0.0)]
    Unimplemented(GroupType),
    // #[error("Unknown error")]
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidInput { field, value } => {
                write!(f, "Invalid input for field \"{field}\": `{value}`")
            }
            Error::Unimplemented(group) => {
                write!(f, "Unimplemented RDS Group Type: {:?}", group.0)
            }
            Error::Unknown => write!(f, "Unknown error"),
        }
    }
}
