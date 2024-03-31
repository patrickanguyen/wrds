//! Decoder for Type 0 Group: Basic tuning and switching information.
//! Fields include Program Service Name (PS) and Alternative Frequency (AF).

use super::{Block2, Block3, Block4, ProgramIdentifier};
use bitflags::bitflags;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MusicSpeechCode {
    Speech,
    MusicOrUnused,
}

impl From<bool> for MusicSpeechCode {
    fn from(value: bool) -> Self {
        match value {
            true => Self::MusicOrUnused,
            false => Self::Speech,
        }
    }
}

bitflags! {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct DecoderIdentifier: u8 {
        const IS_STEREO = 0b0001;
        const IS_ARTIFICIAL_HEAD = 0b0010;
        const IS_COMPRESSED = 0b0100;
        const IS_DYNAMIC_PTY = 0b1000;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrafficProgramCode(bool);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlternativeFreqCode {
    NotToBeUsed,
    Frequency(u32),
    FillerCode,
    NotAssigned,
    NoAfExists,
    AfFollows(usize),
    LfMfFrequencyFollows,
}

impl From<u8> for AlternativeFreqCode {
    fn from(value: u8) -> Self {
        const NOT_TO_BE_USED: u8 = 0;
        const FREQ_MIN: u8 = 1;
        const FREQ_MAX: u8 = 204;
        const FILLER_CODE: u8 = 205;
        const NOT_ASSIGNED1_MIN: u8 = 206;
        const NOT_ASSIGNED1_MAX: u8 = 223;
        const NO_AF_EXISTS: u8 = 224;
        const AF_FOLLOWS_MIN: u8 = 225;
        const AF_FOLLOWS_MAX: u8 = 249;
        const LF_MF_FREQ_FOLLOWS: u8 = 250;
        const NOT_ASSIGNED2_MIN: u8 = 251;
        const NOT_ASSIGNED2_MAX: u8 = 255;

        match value {
            NOT_TO_BE_USED => Self::NotToBeUsed,
            num @ FREQ_MIN..=FREQ_MAX => {
                const BASE_FREQ: u32 = 87600;
                let freq = BASE_FREQ + (100 * ((num - 1) as u32));
                Self::Frequency(freq)
            }
            FILLER_CODE => Self::FillerCode,
            NOT_ASSIGNED1_MIN..=NOT_ASSIGNED1_MAX | NOT_ASSIGNED2_MIN..=NOT_ASSIGNED2_MAX => {
                Self::NotAssigned
            }
            NO_AF_EXISTS => Self::NoAfExists,
            num @ AF_FOLLOWS_MIN..=AF_FOLLOWS_MAX => {
                let following = (num - AF_FOLLOWS_MIN + 1) as usize;
                Self::AfFollows(following)
            }
            LF_MF_FREQ_FOLLOWS => Self::LfMfFrequencyFollows,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PsSegment([char; 2]);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZeroA {
    pub ta: TrafficProgramCode,
    pub ms: MusicSpeechCode,
    pub di: DecoderIdentifier,
    pub af: (AlternativeFreqCode, AlternativeFreqCode),
    pub ps_segment: PsSegment,
}

impl ZeroA {
    /// Decode RDS Message of Group Type 0A
    pub fn new(block2: &Block2, block3: &Block3, block4: &Block4) -> Self {
        let (ta, ms, di) = parse_block2(block2);
        let af = Self::parse_block3(block3);
        let ps_segment = parse_block4(block4);
        Self {
            ta,
            ms,
            di,
            af,
            ps_segment,
        }
    }

    fn parse_block3(block: &Block3) -> (AlternativeFreqCode, AlternativeFreqCode) {
        (
            AlternativeFreqCode::from((block.0 & 0xFF) as u8),
            AlternativeFreqCode::from((block.0 >> 8) as u8),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZeroB {
    pub ta: TrafficProgramCode,
    pub ms: MusicSpeechCode,
    pub di: DecoderIdentifier,
    pub pi: ProgramIdentifier,
    pub ps_segment: PsSegment,
}

impl ZeroB {
    pub fn new(block2: &Block2, block3: &Block3, block4: &Block4) -> ZeroB {
        let (ta, ms, di) = parse_block2(block2);
        let pi = ProgramIdentifier(block3.0);
        let ps_segment = parse_block4(block4);
        Self {
            ta,
            ms,
            di,
            pi,
            ps_segment,
        }
    }
}

pub fn parse_block2(block: &Block2) -> (TrafficProgramCode, MusicSpeechCode, DecoderIdentifier) {
    const BITMASK: u16 = 0x1F;
    let ta_ms_di = block.0 & BITMASK;

    const TA_BITMASK: u16 = 0x10;
    const MS_BITMASK: u16 = 0x08;
    const DI_BITMASK: u16 = 0x07;

    (
        TrafficProgramCode(ta_ms_di & TA_BITMASK != 0),
        MusicSpeechCode::from(ta_ms_di & MS_BITMASK != 0),
        DecoderIdentifier::from_bits((ta_ms_di & DI_BITMASK) as u8)
            .expect("DI should only be 4 bits wide after bitmask"),
    )
}

pub fn parse_block4(block: &Block4) -> PsSegment {
    let letter1 = char::from((block.0 >> 8) as u8);
    let letter2 = char::from((block.0 & 0xFF) as u8);
    PsSegment([letter1, letter2])
}
