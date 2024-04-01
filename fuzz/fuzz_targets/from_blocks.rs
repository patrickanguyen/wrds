#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use libwrds::{from_blocks, Block1, Block2, Block3, Block4};

#[derive(Clone, Debug, Arbitrary)]
pub struct Blocks {
    pub block1: u16,
    pub block2: u16,
    pub block3: u16,
    pub block4: u16,
}

fuzz_target!(|blocks: Blocks| {
    // Should not panic
    _ = from_blocks(&Block1(blocks.block1), &Block2(blocks.block2), &Block3(blocks.block3), &Block4(blocks.block4));
});
