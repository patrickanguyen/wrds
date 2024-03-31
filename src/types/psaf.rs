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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrafficAnnouncementCode(pub bool);

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
pub struct PsSegment(pub [char; 2]);


