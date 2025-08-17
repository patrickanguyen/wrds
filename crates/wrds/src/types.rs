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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// This represents the current state of the RDS metadata that has come in so far.
/// Only the completed metadata is stored within this struct (e.g., incomplete PS segments).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Metadata {
    pub pi: Option<ProgrammeIdentifier>,
    pub pty: Option<ProgrammeType>,
    pub tp: Option<TrafficProgram>,
    pub ps: Option<[u8; 8]>,
}
