//! Decoder for Type 2 Group: RadioText

use super::{Block2, Block3, Block4, ProgramIdentifier};

/// Flag used to clear the screen if a change occurs
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextAB(bool);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextAddressCode(u8);

/// Partial Segment of RadioText
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioTextSegment([char; 2]);

/// Payload for Group Type 2A
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TwoA {
    pub text_ab: TextAB,
    pub text_addr_code: TextAddressCode,
    pub rt_segment: (RadioTextSegment, RadioTextSegment),
}

impl TwoA {
    pub fn new(block2: &Block2, block3: &Block3, block4: &Block4) -> Self {
        let (text_ab, text_addr_code) = parse_block2(block2);
        let rt_segment = (parse_rt_segment(block3.0), parse_rt_segment(block4.0));
        Self {
            text_ab,
            text_addr_code,
            rt_segment,
        }
    }
}

/// Payload for Group Type 2B
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TwoB {
    pub text_ab: TextAB,
    pub text_addr_code: TextAddressCode,
    pub pi: ProgramIdentifier,
    pub rt_segment: RadioTextSegment,
}

impl TwoB {
    pub fn new(block2: &Block2, block3: &Block3, block4: &Block4) -> Self {
        let (text_ab, text_addr_code) = parse_block2(block2);
        let pi = ProgramIdentifier(block3.0);
        let rt_segment = parse_rt_segment(block4.0);
        Self {
            text_ab,
            text_addr_code,
            pi,
            rt_segment,
        }
    }
}

fn parse_block2(block: &Block2) -> (TextAB, TextAddressCode) {
    const BITMASK: u16 = 0x1F;
    let ab_tac = block.0 & BITMASK;

    const AB_BITMASK: u16 = 0x10;
    const TAC_BITMASK: u16 = 0x0F;

    (
        TextAB(ab_tac & AB_BITMASK != 0),
        TextAddressCode((ab_tac & TAC_BITMASK) as u8),
    )
}

fn parse_rt_segment(block: u16) -> RadioTextSegment {
    let letter1 = char::from((block >> 8) as u8);
    let letter2 = char::from((block & 0xFF) as u8);
    RadioTextSegment([letter1, letter2])
}
