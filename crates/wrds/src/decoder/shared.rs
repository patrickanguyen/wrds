use crate::types::{Block2, GroupType, GroupVariant, ProgrammeType, TrafficProgram};

/// Struct containing fields inside Block2 that are always present in all RDS messages regardless of Group Type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shared {
    pub gt: GroupType,
    pub gv: GroupVariant,
    pub tp: TrafficProgram,
    pub pty: ProgrammeType,
}

impl From<Block2> for Shared {
    fn from(value: Block2) -> Self {
        let pty_tp_gv_gt = value.0 >> 5;
        let pty_value: u8 = (pty_tp_gv_gt & 0x1F)
            .try_into()
            .expect("PTY should fit within 8 bits");
        let tp_value = (pty_tp_gv_gt >> 5) & 0x1;
        let gv_value = (pty_tp_gv_gt >> 6) & 0x1;
        let gt_value: u8 = ((pty_tp_gv_gt >> 7) & 0xF)
            .try_into()
            .expect("Group type should fit within 8 bits");

        Self {
            gt: GroupType::try_from(gt_value).expect("Group Type should be a 5-bit value"),
            gv: GroupVariant::from(gv_value != 0),
            tp: TrafficProgram(tp_value != 0),
            pty: ProgrammeType::try_from(pty_value).expect("PTY should be a 4-bit value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Block2, GroupType, GroupVariant, ProgrammeType, Shared, TrafficProgram};

    #[test]
    fn all_set() {
        let block2 = Block2(0xFFFF);
        let shared = Shared::from(block2);

        assert_eq!(
            shared,
            Shared {
                pty: ProgrammeType(0x1F),
                gt: GroupType(0xF),
                gv: GroupVariant::B,
                tp: TrafficProgram(true)
            }
        );
    }

    #[test]
    fn none_set() {
        let block2 = Block2(0x0000);
        let shared = Shared::from(block2);

        assert_eq!(
            shared,
            Shared {
                pty: ProgrammeType(0x00),
                gt: GroupType(0x0),
                gv: GroupVariant::A,
                tp: TrafficProgram(false)
            }
        );
    }

    #[test]
    fn beef() {
        let block2 = Block2(0xBEEF);

        assert_eq!(
            Shared::from(block2),
            Shared {
                pty: ProgrammeType(0x17),
                gt: GroupType(0xB),
                gv: GroupVariant::B,
                tp: TrafficProgram(true),
            }
        )
    }
}
