#![no_main]

use libfuzzer_sys::fuzz_target;
use wrds::{Decoder, Message};

fuzz_target!(|messages: Vec<Message>| {
    let mut decoder = Decoder::new();
    for message in messages {
        let _ = decoder.decode(&message);
    }
});
