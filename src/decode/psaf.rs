//! Decoder for Type 0 Group: Basic tuning and switching information.
//! Fields include Program Service Name (PS) and Alternative Frequency (AF).

use crate::types::{
    psaf::{
        AlternativeFreqCode, DecoderIdentifier, MusicSpeechCode, PsSegment, TrafficAnnouncementCode,
    },
    Block2, Block3, Block4, ProgramIdentifier, ZeroAPayload, ZeroBPayload,
};

fn decode_block2(block: &Block2) -> (TrafficAnnouncementCode, MusicSpeechCode, DecoderIdentifier) {
    const BITMASK: u16 = 0x1F;
    let ta_ms_di = block.0 & BITMASK;

    const TA_BITMASK: u16 = 0x10;
    const MS_BITMASK: u16 = 0x08;
    const DI_BITMASK: u16 = 0x07;

    (
        TrafficAnnouncementCode(ta_ms_di & TA_BITMASK != 0),
        MusicSpeechCode::from(ta_ms_di & MS_BITMASK != 0),
        DecoderIdentifier::from_bits((ta_ms_di & DI_BITMASK) as u8)
            .expect("DI should only be 4 bits wide after bitmask"),
    )
}

fn decode_block3(block: &Block3) -> (AlternativeFreqCode, AlternativeFreqCode) {
    (
        AlternativeFreqCode::from((block.0 & 0xFF) as u8),
        AlternativeFreqCode::from((block.0 >> 8) as u8),
    )
}

fn decode_block4(block: &Block4) -> PsSegment {
    let letter1 = char::from((block.0 >> 8) as u8);
    let letter2 = char::from((block.0 & 0xFF) as u8);
    PsSegment([letter1, letter2])
}

/// Decode RDS Message of Group Type 0A
pub fn decode_0a(block2: &Block2, block3: &Block3, block4: &Block4) -> ZeroAPayload {
    let (ta, ms, di) = decode_block2(block2);
    let af = decode_block3(block3);
    let ps_segment = decode_block4(block4);
    ZeroAPayload {
        ta,
        ms,
        di,
        af,
        ps_segment,
    }
}

/// Decode RDS Message of Group Type 0B
pub fn decode_0b(block2: &Block2, block3: &Block3, block4: &Block4) -> ZeroBPayload {
    let (ta, ms, di) = decode_block2(block2);
    let pi = ProgramIdentifier(block3.0);
    let ps_segment = decode_block4(block4);
    ZeroBPayload {
        ta,
        ms,
        di,
        pi,
        ps_segment,
    }
}
