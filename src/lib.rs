#![cfg_attr(not(test), no_std)]

pub mod parser;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    // use crate::parser::Error;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn basic() {
        let (block1, block2, block3, block4) = (12312, 22, 11, 0xBFFF);

        let parser = parser::Parser::new(parser::RdsStandard::Rds);
        assert!(parser.parse(&block1, &block2, &block3, &block4).is_ok());
    }
}
