use super::{Block1, Block2, GroupType, ProgramIdentifier, ProgramType, TrafficProgramCode};

/// Information that will always be represent in every RDS/RBDS message.
pub struct Shared {
    pub pi: ProgramIdentifier,
    pub group_type: GroupType,
    pub tp: TrafficProgramCode,
    pub pty: ProgramType,
}

impl Shared {
    /// Decode Block1 and Block2 for shared information.
    pub fn new(block1: &Block1, block2: &Block2) -> Self {
        let shared = block2.0 >> 4;
        let pty_value = shared & 0x1F;
        let tp_value = (shared >> 5) & 0x1;
        let gt_value = ((shared >> 6) & 0x1) as u8;

        Self {
            pi: ProgramIdentifier(block1.0),
            group_type: GroupType::n(gt_value).expect("Group Type should be a 4-bit value"),
            tp: tp_value != 0,
            pty: ProgramType(pty_value.try_into().expect("PTY should be a 5-bit value")),
        }
    }
}
