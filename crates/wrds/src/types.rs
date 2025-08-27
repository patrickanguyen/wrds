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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioText {
    rt: heapless::String<MAX_RT_SIZE>,
}

impl RadioText {
    pub fn new(rt: heapless::String<MAX_RT_SIZE>) -> Self {
        Self { rt }
    }

    pub fn as_str(&self) -> &str {
        &self.rt
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
    fn test_message_new() {
        let msg = Message::new(Some(0x1234), None, Some(0xABCD), Some(0xFFFF));
        assert_eq!(msg.block1, Some(Block1(0x1234)));
        assert_eq!(msg.block2, None);
        assert_eq!(msg.block3, Some(Block3(0xABCD)));
        assert_eq!(msg.block4, Some(Block4(0xFFFF)));
    }
}
