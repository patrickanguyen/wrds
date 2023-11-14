pub mod payload;

use core::fmt;
use payload::Payload;

type Block = u16;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput { field: &'static str, value: u16 },
    Unimplemented(GroupType),
    Other,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidInput { field, value } => {
                write!(f, "Invalid input for {}: {}", field, value)
            }
            Error::Unimplemented(group_type) => {
                write!(f, "Unimplemented group type: {:?}", group_type)
            }
            Error::Other => write!(f, "Other error occurred"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, enumn::N)]
#[repr(u8)]
pub enum GroupType {
    ZeroA,
    ZeroB,
    OneA,
    OneB,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, enumn::N)]
#[repr(u8)]
pub enum RdsProgramType {
    Undefined,
    News,
    CurrentAffairs,
    Information,
    Sport,
    Education,
    Drama,
    Culture,
    Science,
    Varied,
    PopMusic,
    RockMusic,
    EasyListening,
    LightClassical,
    SeriousClassical,
    OtherMusic,
    Weather,
    Finance,
    ChildProgrammes,
    SocialAffairs,
    Religion,
    PhoneIn,
    Travel,
    Leisure,
    Jazz,
    CountryMusic,
    NationalMusic,
    OldiesMusic,
    FolkMusic,
    Documentary,
    AlarmTest,
    Alarm,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, enumn::N)]
#[repr(u8)]
pub enum RbdsProgramType {
    Undefined,
    News,
    Information,
    Sports,
    Talk,
    Rock,
    ClassicRock,
    AdultHits,
    SoftRock,
    Top40,
    Country,
    Oldies,
    SoftMusic,
    Nostalgia,
    Jazz,
    Classical,
    RhythmAndBlues,
    SoftRhythmAndBlues,
    Language,
    ReligiousMusic,
    ReligiousTalk,
    Personality,
    Public,
    College,
    SpanishTalk,
    SpanishMusic,
    HipHop,
    Weather = 29,
    EmergencyTest,
    Emergency,
}

impl TryFrom<u8> for RbdsProgramType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match (RbdsProgramType::n(value), value) {
            (Some(pty), _) => Ok(pty),
            (None, 27..=28) => Ok(RbdsProgramType::Undefined),
            (None, value) => Err(Error::InvalidInput {
                field: "RBDS PTY",
                value: u16::from(value),
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramType {
    RDS(RdsProgramType),
    RBDS(RbdsProgramType),
}

pub enum RdsStandard {
    Rds,
    Rbds,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramIdentifier(u16);
pub type TrafficProgramCode = bool;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub pi: ProgramIdentifier,
    pub group_type: GroupType,
    pub tp: TrafficProgramCode,
    pub pty: ProgramType,
    pub payload: Payload,
}

pub struct Parser {
    standard: RdsStandard,
}

impl Parser {
    pub fn new(standard: RdsStandard) -> Self {
        Self { standard }
    }

    pub fn parse(
        &self,
        block1: &Block,
        block2: &Block,
        block3: &Block,
        block4: &Block,
    ) -> Result<Message, Error> {
        let shared = SharedStructure::new(&self.standard, block1, block2)?;
        let payload = Payload::new(&shared.pi, &shared.group_type, block2, block3, block4)?;

        Ok(Message {
            pi: shared.pi,
            group_type: shared.group_type,
            tp: shared.tp,
            pty: shared.pty,
            payload,
        })
    }
}

#[derive(Debug)]
struct SharedStructure {
    pi: ProgramIdentifier,
    group_type: GroupType,
    tp: TrafficProgramCode,
    pty: ProgramType,
}

impl SharedStructure {
    pub fn new(standard: &RdsStandard, block1: &Block, block2: &Block) -> Result<Self, Error> {
        let pi = ProgramIdentifier(*block1);
        let (group_type, tp, pty) = Self::parse_block2(standard, block2)?;
        Ok(Self {
            pi,
            group_type,
            tp,
            pty,
        })
    }

    fn parse_block2(
        standard: &RdsStandard,
        block2: &Block,
    ) -> Result<(GroupType, TrafficProgramCode, ProgramType), Error> {
        const BITSHIFT: usize = 5;
        let gt_tp_pty = block2 >> BITSHIFT;

        let group_type = {
            const GT_BITSHIFT: usize = 7;
            let raw_gt = (gt_tp_pty >> GT_BITSHIFT) as u8;
            GroupType::n(raw_gt).expect("GroupType should be less than 0x1F")
        };

        let tp: TrafficProgramCode = {
            const TP_BITMASK: u16 = 0x20;
            gt_tp_pty & TP_BITMASK != 0
        };

        let pty = {
            const PTY_BITMASK: u16 = 0x1F;
            let raw_pty = (gt_tp_pty & PTY_BITMASK) as u8;
            match standard {
                RdsStandard::Rds => ProgramType::RDS(
                    RdsProgramType::n(raw_pty).expect("RDS PTY should be less than 0x1F"),
                ),
                RdsStandard::Rbds => ProgramType::RBDS(
                    RbdsProgramType::try_from(raw_pty).expect("RBDS PTY should be less than 0x1F"),
                ),
            }
        };

        Ok((group_type, tp, pty))
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use super::{Parser, RdsStandard};

    #[test]
    fn test_parse() {
        let parser = Parser::new(RdsStandard::Rds);
        assert!(true);
    }
}
