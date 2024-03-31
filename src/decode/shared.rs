//! Decoder for RDS shared structure.

use crate::types::{Block1, Block2, GroupType, ProgramIdentifier, ProgramType, TrafficProgramCode};

/// Information that will always be represent in every RDS/RBDS message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shared {
    pub pi: ProgramIdentifier,
    pub group_type: GroupType,
    pub tp: TrafficProgramCode,
    pub pty: ProgramType,
}

impl Shared {
    /// Decode Block1 and Block2 for shared information.
    pub fn new(block1: &Block1, block2: &Block2) -> Self {
        let pty_tp_gt = block2.0 >> 5;
        let pty_value = pty_tp_gt & 0x1F;
        let tp_value = (pty_tp_gt >> 5) & 0x1;
        let gt_value = ((pty_tp_gt >> 6) & 0x1F) as u8;

        Self {
            pi: ProgramIdentifier(block1.0),
            group_type: GroupType::n(gt_value).expect("Group Type should be a 4-bit value"),
            tp: TrafficProgramCode(tp_value != 0),
            pty: ProgramType(pty_value.try_into().expect("PTY should be a 5-bit value")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Block1, Block2, GroupType, ProgramIdentifier, ProgramType, Shared, TrafficProgramCode,
    };

    #[test]
    fn all_set() {
        let block1 = Block1(0xFFFF);
        let block2 = Block2(0xFFFF);

        let shared = Shared::new(&block1, &block2);

        assert_eq!(
            shared,
            Shared {
                pi: ProgramIdentifier(0xFFFF),
                pty: ProgramType(0x1F),
                group_type: GroupType::FifteenB,
                tp: TrafficProgramCode(true)
            }
        );
    }

    #[test]
    fn none_set() {
        let block1 = Block1(0);
        let block2 = Block2(0);

        let shared = Shared::new(&block1, &block2);
        assert_eq!(
            shared,
            Shared {
                pi: ProgramIdentifier(0),
                pty: ProgramType(0),
                group_type: GroupType::ZeroA,
                tp: TrafficProgramCode(false)
            }
        );
    }
}
