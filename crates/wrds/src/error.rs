use crate::types::GroupType;
use thiserror::Error;

/// Radio Data System Decoding Error
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("Invalid input for field \"{field}\": `{value}`")]
    InvalidInput { field: &'static str, value: u16 },
    #[error("Unimplemented RDS Group Type: {:?}", 0.0)]
    Unimplemented(GroupType),
    #[error("Unknown error")]
    Unknown,
}
