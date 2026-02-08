#![deny(unsafe_code)]
// Enforce no_std support when `heapless` feature is enabled,
// except for tests and fuzzing.
#![cfg_attr(
    all(feature = "heapless", not(test), not(feature = "fuzzing"),),
    no_std
)]

mod decoder;
pub use decoder::Decoder;

mod error;
pub use error::Error;

mod types;
pub use types::{
    Message, Metadata, ProgrammeIdentifier, ProgrammeType, RadioText, RadioTextPlusContentType,
    RadioTextPlusTag, TrafficProgram,
};
