use core::fmt;

use self::shared::Shared;

mod psaf;
mod rt;
mod shared;

/// First Block in RDS Message.
/// Contains information about the PI code
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block1(pub u16);

/// Second block in RDS Message.
/// Contains information about Group Type code, Program Type code,
/// and Group Type specific information.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block2(pub u16);

/// Third block in RDS Message.
/// Contains information on either the Group specific payload or
/// PI code, depending on the Message Group Type (A or B).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block3(pub u16);

/// Fourth block in RDS Message.
/// Contains information on the Group specific payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block4(pub u16);

/// Radio Data System Decoding Error
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput { field: &'static str, value: u16 },
    Unimplemented(GroupType),
    Unknown,
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
            Error::Unknown => write!(f, "Unknown error occurred"),
        }
    }
}

/// RDS Message Group Type
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

/// Predefined Program Type to identify different type of
/// programming by genre.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgramType(u8);

/// Unique code that identifies the station.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgramIdentifier(u16);

/// Identifies whether the station broadcasts traffic announcements.
pub type TrafficProgramCode = bool;

/// Decoded Radio Data System Message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub pi: ProgramIdentifier,
    pub group_type: GroupType,
    pub tp: TrafficProgramCode,
    pub pty: ProgramType,
    pub payload: Payload,
}

/// Payload of Radio Data System Message.
/// Contents of the payload depends on the Group Type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Payload {
    ZeroA(psaf::ZeroA),
    ZeroB(psaf::ZeroB),
    OneA,
    OneB,
    TwoA(rt::TwoA),
    TwoB(rt::TwoB),
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
        group_type: &GroupType,
        block2: &Block2,
        block3: &Block3,
        block4: &Block4,
    ) -> Result<Self, Error> {
        match group_type {
            GroupType::ZeroA => Ok(Self::ZeroA(psaf::ZeroA::new(block2, block3, block4))),
            GroupType::ZeroB => Ok(Self::ZeroB(psaf::ZeroB::new(block2, block3, block4))),
            GroupType::TwoA => Ok(Self::TwoA(rt::TwoA::new(block2, block3, block4))),
            GroupType::TwoB => Ok(Self::TwoB(rt::TwoB::new(block2, block3, block4))),
            _ => Err(Error::Unimplemented(*group_type)),
        }
    }
}

/// Decode RDS/RBDS Message from RDS Blocks
pub fn from_blocks(
    block1: &Block1,
    block2: &Block2,
    block3: &Block3,
    block4: &Block4,
) -> Result<Message, Error> {
    let shared = Shared::new(&block1, &block2);

    Ok(Message {
        pi: shared.pi,
        group_type: shared.group_type,
        tp: shared.tp,
        pty: shared.pty,
        payload: Payload::new(&shared.group_type, &block2, &block3, &block4)?,
    })
}
