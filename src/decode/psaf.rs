//! Decoder for Type 0 Group: Basic tuning and switching information.
//! Fields include Program Service Name (PS) and Alternative Frequency (AF).

use crate::types::{
    psaf::{
        AlternativeFreqCode, CharacterSegment, DecoderIdentifierCode, MusicSpeechCode, PsSegment,
        TrafficAnnouncementCode,
    },
    Block2, Block3, Block4, GroupType, ProgramIdentifier, ZeroAPayload, ZeroBPayload,
};

fn decode_block2(
    block: &Block2,
) -> (
    TrafficAnnouncementCode,
    MusicSpeechCode,
    DecoderIdentifierCode,
    CharacterSegment,
) {
    const BITMASK: u16 = 0x1F;
    let ta_ms_di_cs = block.0 & BITMASK;

    const TA_BITMASK: u16 = 0x10;
    const MS_BITMASK: u16 = 0x08;
    const DI_BITMASK: u16 = 0x04;
    const CS_BITMASK: u16 = 0x03;

    (
        TrafficAnnouncementCode(ta_ms_di_cs & TA_BITMASK != 0),
        MusicSpeechCode::from(ta_ms_di_cs & MS_BITMASK != 0),
        DecoderIdentifierCode(ta_ms_di_cs & DI_BITMASK != 0),
        CharacterSegment((ta_ms_di_cs & CS_BITMASK).into()),
    )
}

fn decode_block3(block: &Block3) -> (AlternativeFreqCode, AlternativeFreqCode) {
    (
        AlternativeFreqCode::from((block.0 >> 8) as u8),
        AlternativeFreqCode::from((block.0 & 0xFF) as u8),
    )
}

fn decode_block4(block: &Block4) -> PsSegment {
    let letter1 = char::from((block.0 >> 8) as u8);
    let letter2 = char::from((block.0 & 0xFF) as u8);
    PsSegment([letter1, letter2])
}

/// Decode RDS Message of Group Type 0A
/// Assumes that the Group Type has been already checked to be 0A.
pub fn decode_0a(block2: &Block2, block3: &Block3, block4: &Block4) -> ZeroAPayload {
    debug_assert_eq!(
        GroupType::n((block2.0 >> 11) as u8).unwrap(),
        GroupType::ZeroA,
        "The Group Type must be 0A"
    );

    let (ta, ms, di, cs) = decode_block2(block2);
    let af = decode_block3(block3);
    let ps_segment = decode_block4(block4);
    ZeroAPayload {
        ta,
        ms,
        di,
        cs,
        af,
        ps_segment,
    }
}

/// Decode RDS Message of Group Type 0B.
/// Assumes that the Group Type has been already checked to be 0B.
pub fn decode_0b(block2: &Block2, block3: &Block3, block4: &Block4) -> ZeroBPayload {
    debug_assert_eq!(
        GroupType::n((block2.0 >> 11) as u8).unwrap(),
        GroupType::ZeroB,
        "The Group Type must be 0B"
    );

    let (ta, ms, di, cs) = decode_block2(block2);
    let pi = ProgramIdentifier(block3.0);
    let ps_segment = decode_block4(block4);
    ZeroBPayload {
        ta,
        ms,
        di,
        cs,
        pi,
        ps_segment,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        decode_0a, decode_0b, AlternativeFreqCode, Block2, Block3, Block4, CharacterSegment,
        DecoderIdentifierCode, MusicSpeechCode, ProgramIdentifier, PsSegment,
        TrafficAnnouncementCode, ZeroAPayload, ZeroBPayload,
    };

    #[test]
    fn zero_a() {
        let block2 = Block2(0x0000);
        let block3 = Block3(0x1234);
        let block4 = Block4(0x5678);
        assert_eq!(
            decode_0a(&block2, &block3, &block4),
            ZeroAPayload {
                ta: TrafficAnnouncementCode(false),
                ms: MusicSpeechCode::Speech,
                di: DecoderIdentifierCode(false),
                cs: CharacterSegment(0),
                af: (
                    AlternativeFreqCode::Frequency(89300),
                    AlternativeFreqCode::Frequency(92700)
                ),
                ps_segment: PsSegment(['V', 'x']),
            }
        )
    }

    #[test]
    fn zero_b() {
        let block2 = Block2(0x0800);
        let block3 = Block3(0x1234);
        let block4 = Block4(0x5678);

        assert_eq!(
            decode_0b(&block2, &block3, &block4),
            ZeroBPayload {
                ta: TrafficAnnouncementCode(false),
                ms: MusicSpeechCode::Speech,
                di: DecoderIdentifierCode(false),
                cs: CharacterSegment(0),
                ps_segment: PsSegment(['V', 'x']),
                pi: ProgramIdentifier(0x1234)
            }
        )
    }
}
