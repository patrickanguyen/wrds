//! Types relating to Type 0 Group: Basic tuning and switching information.

use bitflags::bitflags;

/// Indication of whether music or speech is being broadcasted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MusicSpeechCode {
    /// Speech, at present, is being broadcasted.
    Speech,
    /// Music, at present, is being broadcasted.
    /// This could also indicate that the broadcaster is not using this field.
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

/// Used in conduction with TrafficProgramCode to indicate whether a traffic announcement is
/// currently being broadcasted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrafficAnnouncementCode(pub bool);

/// Individual bit of the Decoder Identifier.
///
/// DecoderIdentifierCode is sent progressively over 4 messages with CharacterSegment to determine the location
/// in the overall DecoderIdentifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecoderIdentifierCode(pub bool);

/// Determines the location of the DecoderIdentifierCode in the DecoderIdentifier codeword
/// and the Programme Service name (PS) segment in the overall Programme Service name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharacterSegment(pub usize);

bitflags! {
    /// Used to indicate different operating modes.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct DecoderIdentifier: u8 {
        /// Is stereo or mono?
        const IS_STEREO = 0b0001;
        /// Is Artificial Head or not?
        const IS_ARTIFICIAL_HEAD = 0b0010;
        /// Is compressed or not?
        const IS_COMPRESSED = 0b0100;
        /// Is PTY dynamically switched or static PTY?
        ///
        const IS_DYNAMIC_PTY = 0b1000;
    }
}

/// Alternative Frequency Table Code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlternativeFreqCode {
    /// Not to be used.
    NotToBeUsed,
    /// Frequency.
    Frequency(u32),
    /// Filler code.
    FillerCode,
    /// Not assigned.
    NotAssigned,
    /// No AF exists.
    NoAfExists,
    /// N AF Follows.
    AfFollows(usize),
    /// An LF/MF Frequency follows.
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

/// Segment of Programme Service name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PsSegment(pub [char; 2]);
