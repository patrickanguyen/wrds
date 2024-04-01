//! # WRDS
//!
//! WRDS is a Radio Data System or RDS Decoder (<https://en.wikipedia.org/wiki/Radio_Data_System>).
//!
//! RDS is commonly used in FM receivers to receive metadata about the currently tuned station.
//!
//! WRDS is meant to serve as a platform-agnostic and hardware-agnostic decoding library for your FM receiver application.
//!
//! Note that this decode is state-less, so it does not keep track of the previously received RDS messages.
//! It is the responsibility of your application to keep track of state of the currently tuned program.
//!
//! ```
//! use libwrds::{from_blocks, Block1, Block2, Block3, Block4};
//!
//! let message = from_blocks(&Block1(5), &Block2(25), &Block3(55), &Block4(44));
//! assert!(message.is_ok());
//! ```
//!

#![cfg_attr(not(test), no_std)]

pub mod decode;
pub mod error;
pub mod types;

pub use decode::from_blocks;
pub use types::{Block1, Block2, Block3, Block4, Message};

#[cfg(test)]
mod tests {
    use super::{from_blocks, Block1, Block2, Block3, Block4};

    #[test]
    fn test_gt0() {
        let message = from_blocks(&Block1(5), &Block2(25), &Block3(55), &Block4(44));
        assert!(message.is_ok());
    }
}
