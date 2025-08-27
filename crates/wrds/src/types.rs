use crate::error::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block1(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block2(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block3(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block4(pub u16);

/// Struct containing all of the RDS/RBDS blocks
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Message {
    pub block1: Option<Block1>,
    pub block2: Option<Block2>,
    pub block3: Option<Block3>,
    pub block4: Option<Block4>,
}

impl Message {
    /// Create RDS Blocks struct.
    /// Option<u16> is used in order to represent whether there are too many bit errors for the block to be used.
    /// For example, if there are too many bit errors for block1 to be used, None should be used.
    pub fn new(
        block1: Option<u16>,
        block2: Option<u16>,
        block3: Option<u16>,
        block4: Option<u16>,
    ) -> Self {
        Self {
            block1: block1.map(Block1),
            block2: block2.map(Block2),
            block3: block3.map(Block3),
            block4: block4.map(Block4),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgrammeIdentifier(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrafficProgram(pub bool);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgrammeType(pub u8);

impl TryFrom<u8> for ProgrammeType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const MAX_VALUE: u8 = 32;

        if value > MAX_VALUE {
            return Err(Error::InvalidInput {
                field: "Group Type must be a 5-bit value",
                value: value.into(),
            });
        }
        Ok(ProgrammeType(value))
    }
}

/// All RDS messages are either A or B variant.
/// If the message is type A, the PI is transmitted only on Block 1.
/// Otherwise, the PI is transmitted both on Block 1 and Block 3.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GroupVariant {
    A,
    B,
}

impl From<bool> for GroupVariant {
    fn from(value: bool) -> Self {
        match value {
            true => GroupVariant::B,
            false => GroupVariant::A,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GroupType(pub u8);

impl TryFrom<u8> for GroupType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const MAX_VALUE: u8 = 15;

        if value > MAX_VALUE {
            return Err(Error::InvalidInput {
                field: "Group Type must be a 4-bit value",
                value: value.into(),
            });
        }

        Ok(GroupType(value))
    }
}

pub const PS_SIZE: usize = 8;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProgrammeServiceName {
    ps: heapless::String<PS_SIZE>,
}

impl ProgrammeServiceName {
    pub fn new(ps: heapless::String<PS_SIZE>) -> Self {
        Self { ps }
    }

    pub fn as_str(&self) -> &str {
        &self.ps
    }
}

/// Max size of Group A RadioText messages
pub const MAX_RT_SIZE: usize = 64;

pub const MAX_RT_PLUS_TAGS: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioText {
    rt: heapless::String<MAX_RT_SIZE>,
    rt_plus: heapless::Vec<RadioTextPlusTag, MAX_RT_PLUS_TAGS>,
}

impl RadioText {
    pub fn new(
        rt: heapless::String<MAX_RT_SIZE>,
        rt_plus: heapless::Vec<RadioTextPlusTag, MAX_RT_PLUS_TAGS>,
    ) -> Self {
        Self { rt, rt_plus }
    }

    pub fn as_str(&self) -> &str {
        &self.rt
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RadioTextPlusTag {
    content_type: RadioTextPlusContentType,
    start_index: usize,
    length: usize,
}

impl RadioTextPlusTag {
    pub fn new(content_type: RadioTextPlusContentType, start_index: usize, length: usize) -> Self {
        Self {
            content_type,
            start_index,
            length,
        }
    }

    pub fn content_type(&self) -> RadioTextPlusContentType {
        self.content_type
    }

    pub fn start_index(&self) -> usize {
        self.start_index
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

/// Content types for RadioTextPlus (RT+).
///
/// Radio Text Plus (RT+) is an extension of RadioText (RT) that enable receivers
/// to identify specific types of information within the RadioText string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RadioTextPlusContentType {
    Dummy,
    Title,
    Album,
    TrackNumber,
    Artist,
    Composition,
    Movement,
    Conductor,
    Composer,
    Band,
    Comment,
    Genre,
    News,
    NewsLocal,
    StockMarket,
    Sport,
    Lottery,
    Horoscope,
    DailyDiversion,
    Health,
    Event,
    Scene,
    Cinema,
    Tv,
    DateTime,
    Weather,
    Traffic,
    Alarm,
    Advertisement,
    Url,
    Other,
    ShortStationName,
    LongStationName,
    NowProgramme,
    NextProgramme,
    ProgrammePart,
    ProgrammeHost,
    ProgrammeEditorialStaff,
    ProgrammeFrequency,
    ProgrammeHomepage,
    ProgrammeSubchannel,
    PhoneHotline,
    PhoneStudio,
    PhoneOther,
    SmsStudio,
    SmsOther,
    EmailHotline,
    EmailStudio,
    EmailOther,
    MmsOther,
    Chat,
    ChatCentre,
    VoteQuestion,
    VoteCentre,
    Place,
    Appointment,
    Identifier,
    Purchase,
    GetData,
}

impl TryFrom<u8> for RadioTextPlusContentType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RadioTextPlusContentType::Dummy),
            1 => Ok(RadioTextPlusContentType::Title),
            2 => Ok(RadioTextPlusContentType::Album),
            3 => Ok(RadioTextPlusContentType::TrackNumber),
            4 => Ok(RadioTextPlusContentType::Artist),
            5 => Ok(RadioTextPlusContentType::Composition),
            6 => Ok(RadioTextPlusContentType::Movement),
            7 => Ok(RadioTextPlusContentType::Conductor),
            8 => Ok(RadioTextPlusContentType::Composer),
            9 => Ok(RadioTextPlusContentType::Band),
            10 => Ok(RadioTextPlusContentType::Comment),
            11 => Ok(RadioTextPlusContentType::Genre),
            12 => Ok(RadioTextPlusContentType::News),
            13 => Ok(RadioTextPlusContentType::NewsLocal),
            14 => Ok(RadioTextPlusContentType::StockMarket),
            15 => Ok(RadioTextPlusContentType::Sport),
            16 => Ok(RadioTextPlusContentType::Lottery),
            17 => Ok(RadioTextPlusContentType::Horoscope),
            18 => Ok(RadioTextPlusContentType::DailyDiversion),
            19 => Ok(RadioTextPlusContentType::Health),
            20 => Ok(RadioTextPlusContentType::Event),
            21 => Ok(RadioTextPlusContentType::Scene),
            22 => Ok(RadioTextPlusContentType::Cinema),
            23 => Ok(RadioTextPlusContentType::Tv),
            24 => Ok(RadioTextPlusContentType::DateTime),
            25 => Ok(RadioTextPlusContentType::Weather),
            26 => Ok(RadioTextPlusContentType::Traffic),
            27 => Ok(RadioTextPlusContentType::Alarm),
            28 => Ok(RadioTextPlusContentType::Advertisement),
            29 => Ok(RadioTextPlusContentType::Url),
            30 => Ok(RadioTextPlusContentType::Other),
            31 => Ok(RadioTextPlusContentType::ShortStationName),
            32 => Ok(RadioTextPlusContentType::LongStationName),
            33 => Ok(RadioTextPlusContentType::NowProgramme),
            34 => Ok(RadioTextPlusContentType::NextProgramme),
            35 => Ok(RadioTextPlusContentType::ProgrammePart),
            36 => Ok(RadioTextPlusContentType::ProgrammeHost),
            37 => Ok(RadioTextPlusContentType::ProgrammeEditorialStaff),
            38 => Ok(RadioTextPlusContentType::ProgrammeFrequency),
            39 => Ok(RadioTextPlusContentType::ProgrammeHomepage),
            40 => Ok(RadioTextPlusContentType::ProgrammeSubchannel),
            41 => Ok(RadioTextPlusContentType::PhoneHotline),
            42 => Ok(RadioTextPlusContentType::PhoneStudio),
            43 => Ok(RadioTextPlusContentType::PhoneOther),
            44 => Ok(RadioTextPlusContentType::SmsStudio),
            45 => Ok(RadioTextPlusContentType::SmsOther),
            46 => Ok(RadioTextPlusContentType::EmailHotline),
            47 => Ok(RadioTextPlusContentType::EmailStudio),
            48 => Ok(RadioTextPlusContentType::EmailOther),
            49 => Ok(RadioTextPlusContentType::MmsOther),
            50 => Ok(RadioTextPlusContentType::Chat),
            51 => Ok(RadioTextPlusContentType::ChatCentre),
            52 => Ok(RadioTextPlusContentType::VoteQuestion),
            53 => Ok(RadioTextPlusContentType::VoteCentre),
            // 54-59 are reserved or private
            59 => Ok(RadioTextPlusContentType::Place),
            60 => Ok(RadioTextPlusContentType::Appointment),
            61 => Ok(RadioTextPlusContentType::Identifier),
            62 => Ok(RadioTextPlusContentType::Purchase),
            63 => Ok(RadioTextPlusContentType::GetData),
            _ => Err(Error::InvalidInput {
                field: "RadioTextPlusTag must a value within 0-53, 59-63",
                value: value.into(),
            }),
        }
    }
}

/// This represents the current state of the RDS metadata that has come in so far.
/// Only the completed metadata is stored within this struct (e.g., incomplete PS segments).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Metadata {
    pub pi: Option<ProgrammeIdentifier>,
    pub pty: Option<ProgrammeType>,
    pub tp: Option<TrafficProgram>,
    pub ps: Option<ProgrammeServiceName>,
    pub rt: Option<RadioText>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_programme_type_try_from() {
        assert_eq!(ProgrammeType::try_from(0).unwrap(), ProgrammeType(0));
        assert_eq!(ProgrammeType::try_from(15).unwrap(), ProgrammeType(15));
        assert!(ProgrammeType::try_from(33).is_err());
    }

    #[test]
    fn test_group_type_try_from() {
        assert_eq!(GroupType::try_from(0).unwrap(), GroupType(0));
        assert_eq!(GroupType::try_from(15).unwrap(), GroupType(15));
        assert!(GroupType::try_from(16).is_err());
    }

    #[test]
    fn test_radio_text_plus_content_type_try_from() {
        assert_eq!(
            RadioTextPlusContentType::try_from(0).unwrap(),
            RadioTextPlusContentType::Dummy
        );
        assert_eq!(
            RadioTextPlusContentType::try_from(53).unwrap(),
            RadioTextPlusContentType::VoteCentre
        );
        assert_eq!(
            RadioTextPlusContentType::try_from(59).unwrap(),
            RadioTextPlusContentType::Place
        );
        assert_eq!(
            RadioTextPlusContentType::try_from(63).unwrap(),
            RadioTextPlusContentType::GetData
        );
        assert!(RadioTextPlusContentType::try_from(54).is_err());
        assert!(RadioTextPlusContentType::try_from(64).is_err());
    }

    #[test]
    fn test_message_new() {
        let msg = Message::new(Some(0x1234), None, Some(0xABCD), Some(0xFFFF));
        assert_eq!(msg.block1, Some(Block1(0x1234)));
        assert_eq!(msg.block2, None);
        assert_eq!(msg.block3, Some(Block3(0xABCD)));
        assert_eq!(msg.block4, Some(Block4(0xFFFF)));
    }
}
