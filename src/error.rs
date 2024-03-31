use core::fmt;
use crate::types::GroupType;

/// Radio Data System Decoding Error
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput { field: &'static str, value: u16 },
    Unimplemented(GroupType),
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidInput { field, value } => {
                write!(f, "Invalid input for {}: {}", field, value)
            }
            Error::Unimplemented(group_type) => {
                write!(f, "Unimplemented group type: {:?}", group_type)
            }
            Error::Unknown => write!(f, "Unknown error occurred"),
        }
    }
}
