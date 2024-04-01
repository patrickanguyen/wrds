use crate::types::GroupType;

/// Radio Data System Decoding Error
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput { field: &'static str, value: u16 },
    Unimplemented(GroupType),
    Unknown,
}
