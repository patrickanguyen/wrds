//! Decoder for Type 2 Group: RadioText.

use crate::types::{
    rt::{RadioTextSegment, TextAB, TextAddressCode},
    Block2, Block3, Block4, GroupType, ProgramIdentifier, TwoAPayload, TwoBPayload,
};

fn decode_block2(block: &Block2) -> (TextAB, TextAddressCode) {
    const BITMASK: u16 = 0x1F;
    let ab_tac = block.0 & BITMASK;

    const AB_BITMASK: u16 = 0x10;
    const TAC_BITMASK: u16 = 0x0F;

    (
        TextAB(ab_tac & AB_BITMASK != 0),
        TextAddressCode((ab_tac & TAC_BITMASK) as u8),
    )
}

fn decode_rt_segment(block: u16) -> RadioTextSegment {
    let letter1 = char::from((block >> 8) as u8);
    let letter2 = char::from((block & 0xFF) as u8);
    RadioTextSegment([letter1, letter2])
}

/// Decode RDS Message of Group Type 2A.
/// Assumes that the Group Type has been already checked to be 2A.
pub fn decode_2a(block2: &Block2, block3: &Block3, block4: &Block4) -> TwoAPayload {
    debug_assert_eq!(
        GroupType::n((block2.0 >> 11) as u8).unwrap(),
        GroupType::TwoA,
        "The Group Type must be 2A"
    );

    let (text_ab, text_addr_code) = decode_block2(block2);
    let rt_segment = (decode_rt_segment(block3.0), decode_rt_segment(block4.0));
    TwoAPayload {
        text_ab,
        text_addr_code,
        rt_segment,
    }
}

/// Decode RDS Message of Group Type 2B.
/// Assumes that the Group Type has been already checked to be 2B.
pub fn decode_2b(block2: &Block2, block3: &Block3, block4: &Block4) -> TwoBPayload {
    debug_assert_eq!(
        GroupType::n((block2.0 >> 11) as u8).unwrap(),
        GroupType::TwoB,
        "The Group Type must be 2B"
    );

    let (text_ab, text_addr_code) = decode_block2(block2);
    let pi = ProgramIdentifier(block3.0);
    let rt_segment = decode_rt_segment(block4.0);
    TwoBPayload {
        text_ab,
        text_addr_code,
        pi,
        rt_segment,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        decode_2a, decode_2b, Block2, Block3, Block4, ProgramIdentifier, RadioTextSegment, TextAB,
        TextAddressCode, TwoAPayload, TwoBPayload,
    };

    #[test]
    fn two_a() {
        let block2 = Block2(0x2000);
        let block3 = Block3(0x3234);
        let block4 = Block4(0x5678);

        assert_eq!(
            decode_2a(&block2, &block3, &block4),
            TwoAPayload {
                text_ab: TextAB(false),
                text_addr_code: TextAddressCode(0),
                rt_segment: (RadioTextSegment(['2', '4']), RadioTextSegment(['V', 'x']))
            }
        )
    }

    #[test]
    fn two_b() {
        let block2 = Block2(0x2A00);
        let block3 = Block3(0x3234);
        let block4 = Block4(0x5678);

        assert_eq!(
            decode_2b(&block2, &block3, &block4),
            TwoBPayload {
                text_ab: TextAB(false),
                text_addr_code: TextAddressCode(0),
                rt_segment: RadioTextSegment(['V', 'x']),
                pi: ProgramIdentifier(0x3234)
            }
        )
    }
}
