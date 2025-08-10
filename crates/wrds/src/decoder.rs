use crate::types::{
    Block1, Block2, Block3, Block4, GroupVariant, Message, Metadata, ProgrammeIdentifier
};

use self::shared::Shared;

mod shared;

pub struct Decoder {
    metadata: Metadata,
}

impl Decoder {
    /// Create new RDS decoder.
    pub fn new() -> Self {
        Decoder {
            metadata: Metadata::default(),
        }
    }

    /// Decode the RDS message and return the current state of the RDS metadata.
    pub fn decode(&mut self, blocks: &Message) -> &Metadata {
        self.decode_block1(&blocks.block1);

        // Return immediately if Block 2 is not provided because it determines
        // how to decode Block 3 and 4.
        if blocks.block2.is_none() {
            return &self.metadata;
        }

        self.decode_blocks234(
            &blocks.block2.expect("Block 2 should not be empty"),
            &blocks.block3,
            &blocks.block4,
        );

        &self.metadata
    }

    /// Reset Decoder's state to default.
    /// This method should be called after tuning to a different station.
    pub fn reset(&mut self) {
        self.metadata = Metadata::default();
    }

    /// Decode Block 1 as the Programme Identifier (PI) if provided.
    fn decode_block1(&mut self, block1: &Option<Block1>) {
        self.metadata.pi = block1.map(|block| ProgrammeIdentifier(block.0));
    }

    /// Decode Blocks 2, 3, and 4.
    /// Block 2 must be provided because it determines how to decode Blocks 3 and 4.
    fn decode_blocks234(
        &mut self,
        block2: &Block2,
        block3: &Option<Block3>,
        _block4: &Option<Block4>,
    ) {
        let shared = Shared::from(*block2);
        // If PI cannot be retrieved from Block 1 and Group Variant is Type B, use PI from Block 3.
        if self.metadata.pi.is_none() && (shared.gv == GroupVariant::B && block3.is_some()) {
            self.metadata.pi = Some(ProgrammeIdentifier(
                block3.expect("Block 3 should not be empty").0,
            ));
        }
        self.metadata.pty = Some(shared.pty);
        self.metadata.tp = Some(shared.tp);
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
