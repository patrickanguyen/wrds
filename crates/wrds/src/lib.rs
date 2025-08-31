#![deny(unsafe_code)]
#![cfg_attr(all(not(test), not(feature = "fuzzing")), no_std)]

mod decoder;
pub use decoder::Decoder;

mod error;
pub use error::Error;

mod types;
pub use types::{
    Message, Metadata, ProgrammeIdentifier, ProgrammeType, RadioText, RadioTextPlusContentType,
    RadioTextPlusTag, TrafficProgram,
};
