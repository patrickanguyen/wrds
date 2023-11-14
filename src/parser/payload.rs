use super::{Block, Error, GroupType, ProgramIdentifier};
use bitflags::bitflags;

#[derive(Clone, Debug, PartialEq, Eq)]
enum AlternativeFreqCode {
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
pub enum Payload {
    ZeroA(zero::A),
    ZeroB(zero::B),
    OneA(one::A),
    OneB(one::B),
    TwoA,
    TwoB,
    ThreeA,
    ThreeB,
    FourA,
    FourB,
    FiveA,
    FiveB,
    SixA,
    SixB,
    SevenA,
    SevenB,
    EightA,
    EightB,
    NineA,
    NineB,
    TenA,
    TenB,
    ElevenA,
    ElevenB,
    TwelveA,
    TwelveB,
    ThirteenA,
    ThirteenB,
}

impl Payload {
    pub fn new(
        pi: &ProgramIdentifier,
        group_type: &GroupType,
        block2: &Block,
        block3: &Block,
        block4: &Block,
    ) -> Result<Self, Error> {
        match group_type {
            GroupType::ZeroA => Ok(Self::ZeroA(zero::A::new(block2, block3, block4))),
            GroupType::ZeroB => Ok(Self::ZeroB(zero::B::new(pi, block2, block3, block4)?)),
            GroupType::OneA => Ok(Self::OneA(one::A::new(block2, block3, block4)?)),
            GroupType::OneB => Ok(Self::OneB(one::B::new(block4)?)),
            _ => Err(Error::Unimplemented(*group_type)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum MusicSpeechCode {
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
    pub struct DecoderIdentifer : u8 {
        const IS_STEREO = 0b0001;
        const IS_ARTIFICAL_HEAD = 0b0010;
        const IS_COMPRESSED = 0b0100;
        const IS_DYNAMIC_PTY = 0b1000;
    }
}

pub type TrafficProgramCode = bool;

pub mod zero {
    use super::{
        AlternativeFreqCode, Block, DecoderIdentifer, Error, MusicSpeechCode, ProgramIdentifier,
        TrafficProgramCode,
    };

    fn parse_block2(block2: &Block) -> (bool, MusicSpeechCode, DecoderIdentifer) {
        const BITMASK: u16 = 0x1F;
        let ta_ms_di = block2 & BITMASK;

        const TA_BITMASK: u16 = 0x10;
        const MS_BITMASK: u16 = 0x08;
        const DI_BITMASK: u16 = 0x07;

        (
            ta_ms_di & TA_BITMASK != 0,
            MusicSpeechCode::from(ta_ms_di & MS_BITMASK != 0),
            DecoderIdentifer::from_bits((ta_ms_di & DI_BITMASK) as u8)
                .expect("DI should only be 4 bits after bitmask"),
        )
    }

    fn parse_block4(block4: &Block) -> [char; 2] {
        const PS1_BITSHIFT: usize = 8;
        const PS2_BITMASK: u16 = 0xFF;
        let raw_ps1 = (block4 >> PS1_BITSHIFT) as u8;
        let raw_ps2 = (block4 & PS2_BITMASK) as u8;

        [char::from(raw_ps1), char::from(raw_ps2)]
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct A {
        ta: TrafficProgramCode,
        ms: MusicSpeechCode,
        di: DecoderIdentifer,
        alternative_freq: (AlternativeFreqCode, AlternativeFreqCode),
        ps_segment: [char; 2],
    }

    impl A {
        pub fn new(block2: &Block, block3: &Block, block4: &Block) -> Self {
            let (ta, ms, di) = parse_block2(block2);
            let alternative_freq = Self::parse_block3(block3);
            let ps_segment = parse_block4(block4);
            Self {
                ta,
                ms,
                di,
                alternative_freq,
                ps_segment,
            }
        }

        fn parse_block3(block3: &Block) -> (AlternativeFreqCode, AlternativeFreqCode) {
            const AF1_BITSHIFT: usize = 8;
            const AF2_BITMASK: u16 = 0xFF;

            (
                AlternativeFreqCode::from((block3 >> AF1_BITSHIFT) as u8),
                AlternativeFreqCode::from((block3 & AF2_BITMASK) as u8),
            )
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct B {
        ta: TrafficProgramCode,
        ms: MusicSpeechCode,
        di: DecoderIdentifer,
        ps_segment: [char; 2],
    }

    impl B {
        pub fn new(
            pi: &ProgramIdentifier,
            block2: &Block,
            block3: &Block,
            block4: &Block,
        ) -> Result<Self, Error> {
            let (ta, ms, di) = parse_block2(block2);
            if pi.0 != *block3 {
                return Err(Error::InvalidInput {
                    field: "Block3 ProgramIdentifier",
                    value: *block3,
                });
            }
            let ps_segment = parse_block4(block4);
            Ok(Self {
                ta,
                ms,
                di,
                ps_segment,
            })
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, enumn::N)]
#[repr(u8)]
pub enum TransmitterNetworkGroup {
    NoBasicPaging,
    Zero99,
    Zero39,
    Forty99,
    Forty69,
    Seventy99,
    Zero19,
    Twenty39,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Interval(u8);

impl TryFrom<u8> for Interval {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const MAX_VALUE: u8 = 0b11;
        if value > MAX_VALUE {
            return Err(Error::InvalidInput {
                field: "Interval",
                value: u16::from(value),
            });
        }
        Ok(Self(value))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioPagingCode {
    tng: TransmitterNetworkGroup,
    interval: Interval,
}

impl TryFrom<u8> for RadioPagingCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const MAX_VALUE: u8 = 0b1_1111;
        if value > MAX_VALUE {
            return Err(Error::InvalidInput {
                field: "RadioPagingCode",
                value: u16::from(value),
            });
        }
        const TNG_BITSHIFT: usize = 2;
        const TNG_BITMASK: u8 = 0b111;
        Ok(Self {
            tng: TransmitterNetworkGroup::n((value >> TNG_BITSHIFT) & TNG_BITMASK)
                .expect("TNG should be less than 0x7"),
            interval: Interval::try_from(value & 0b11).expect("Interval should be less than 0x3"),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlowLabelingCode(u16);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Day(u8);

impl TryFrom<u8> for Day {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const MAX_VALUE: u8 = 31;
        if value > MAX_VALUE {
            return Err(Error::InvalidInput {
                field: "Day",
                value: u16::from(value),
            });
        }
        Ok(Self(value))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hour(u8);

impl TryFrom<u8> for Hour {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const MAX_VALUE: u8 = 23;
        if value > MAX_VALUE {
            return Err(Error::InvalidInput {
                field: "Hour",
                value: u16::from(value),
            });
        }
        Ok(Self(value))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Minute(u8);

impl TryFrom<u8> for Minute {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const MAX_VALUE: u8 = 59;
        if value > MAX_VALUE {
            return Err(Error::InvalidInput {
                field: "Minute",
                value: u16::from(value),
            });
        }
        Ok(Self(value))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramItemNumber {
    day: Day,
    hour: Hour,
    minute: Minute,
}

impl TryFrom<Block> for ProgramItemNumber {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let minute = {
            const BITMASK: u16 = 0x3F;
            Minute::try_from((value & BITMASK) as u8)?
        };
        let value = value >> 6;
        let hour = {
            const BITMASK: u16 = 0x1F;
            Hour::try_from((value & BITMASK) as u8)?
        };
        let value = value >> 5;
        let day = Day::try_from(value as u8)?;
        Ok(Self { day, hour, minute })
    }
}

pub mod one {
    use super::{Block, Day, Error, ProgramItemNumber, RadioPagingCode, SlowLabelingCode};

    fn parse_block4(block4: &Block) -> Result<Option<ProgramItemNumber>, Error> {
        let pin = ProgramItemNumber::try_from(*block4)?;
        if pin.day == Day(0) {
            return Ok(None);
        }
        Ok(Some(pin))
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct A {
        radio_paging_code: RadioPagingCode,
        slow_labelling_code: SlowLabelingCode,
        pin: Option<ProgramItemNumber>,
    }

    impl A {
        pub fn new(block2: &Block, block3: &Block, block4: &Block) -> Result<Self, Error> {
            let radio_paging_code = {
                const BITMASK: u16 = 0b1_1111;
                RadioPagingCode::try_from((*block2 & BITMASK) as u8)
                    .expect("RadioPagingCode should be less than 0x1F")
            };
            let slow_labelling_code = SlowLabelingCode(*block3);
            let pin = parse_block4(block4)?;

            Ok(Self {
                radio_paging_code,
                slow_labelling_code,
                pin,
            })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct B {
        pin: Option<ProgramItemNumber>,
    }

    impl B {
        pub fn new(block4: &Block) -> Result<Self, Error> {
            let pin = parse_block4(block4)?;
            Ok(Self { pin })
        }
    }
}
