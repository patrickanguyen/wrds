// SPDX-FileCopyrightText: 2026 Patrick Nguyen
//
// SPDX-License-Identifier: MPL-2.0

#![no_main]

use libfuzzer_sys::fuzz_target;
use wrds::{Decoder, Message};

fuzz_target!(|messages: Vec<Message>| {
    let mut decoder = Decoder::new();
    for message in messages {
        let _ = decoder.decode(&message);
    }
});
