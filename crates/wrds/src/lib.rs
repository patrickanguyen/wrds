// SPDX-FileCopyrightText: 2026 Patrick Nguyen
//
// SPDX-License-Identifier: MPL-2.0

//! A [Radio Data System (RDS)](https://en.wikipedia.org/wiki/Radio_Data_System) decoder library written in Rust.
//!
//! RDS is a communications protocol that allows FM broadcasts to transmit metadata to receivers like program identification, traffic announcements, and program information.
//!
//! This library supports both `std` and `no_std` (with the `heapless` feature enabled) environments.

#![deny(unsafe_code)]
// Enforce no_std support when `heapless` feature is enabled,
// except for tests and fuzzing.
#![cfg_attr(
    all(feature = "heapless", not(test), not(feature = "fuzzing"),),
    no_std
)]

pub(crate) mod bitset;

mod decoder;
pub use decoder::Decoder;

mod error;

mod types;
pub use types::{
    Message, Metadata, ProgrammeIdentification, ProgrammeServiceName, ProgrammeType, RadioText,
    RadioTextPlusContentType, RadioTextPlusTag, TrafficProgram,
};
