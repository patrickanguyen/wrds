#![cfg_attr(not(test), no_std)]

pub mod decode;
pub mod error;
pub mod types;

#[cfg(test)]
mod tests {
    use super::{
        decode::from_blocks,
        types::{Block1, Block2, Block3, Block4},
    };

    #[test]
    fn test_gt0() {
        let block1 = Block1(5);
        let block2 = Block2(25);
        let block3 = Block3(55);
        let block4 = Block4(44);

        let message = from_blocks(&block1, &block2, &block3, &block4);
        assert!(message.is_ok());
    }
}
