use crate::{
    decoder::{mode_filter::ModeFilter, ps_decoder::PsDecoder},
    types::{Block1, Block2, Block3, Block4, GroupVariant, Message, Metadata, ProgrammeIdentifier},
    ProgrammeType, TrafficProgram,
};

use self::shared::Shared;

mod bitset;
mod mode_filter;
mod ps_decoder;
mod shared;

const PI_FILTER_COUNT: usize = 6;
const PI_FILTER_MIN: usize = 5;

const PTY_FILTER_COUNT: usize = 6;
const PTY_FILTER_MIN: usize = 5;

const TP_FILTER_COUNT: usize = 6;
const TP_FILTER_MIN: usize = 5;

#[derive(Debug)]
pub struct Decoder {
    pi_filter: ModeFilter<ProgrammeIdentifier, PI_FILTER_COUNT>,
    pty_filter: ModeFilter<ProgrammeType, PTY_FILTER_COUNT>,
    tp_filter: ModeFilter<TrafficProgram, TP_FILTER_COUNT>,
    ps_filter: PsDecoder,
}

impl Decoder {
    /// Create new RDS decoder.
    pub fn new() -> Self {
        Decoder {
            pi_filter: ModeFilter::new(PI_FILTER_MIN).unwrap(),
            pty_filter: ModeFilter::new(PTY_FILTER_MIN).unwrap(),
            tp_filter: ModeFilter::new(TP_FILTER_MIN).unwrap(),
            ps_filter: PsDecoder::new(),
        }
    }

    /// Decode the RDS message and return the current state of the RDS metadata.
    pub fn decode(&mut self, blocks: &Message) -> Metadata {
        self.decode_block1(&blocks.block1);

        // Return immediately if Block 2 is not provided because it determines
        // how to decode Block 3 and 4.
        if let Some(block2) = blocks.block2 {
            self.decode_blocks234(&block2, &blocks.block3, &blocks.block4);
        }

        self.metadata()
    }

    /// Reset Decoder's state to default.
    /// This method should be called after tuning to a different station.
    pub fn reset(&mut self) {
        self.pi_filter.reset();
        self.pty_filter.reset();
        self.tp_filter.reset();
    }

    /// Decode Block 1 as the Programme Identifier (PI) if provided.
    fn decode_block1(&mut self, block1: &Option<Block1>) {
        let maybe_pi = block1.map(|block| ProgrammeIdentifier(block.0));
        if let Some(pi) = maybe_pi {
            self.pi_filter.push(pi);
        }
    }

    /// Decode Blocks 2, 3, and 4.
    /// Block 2 must be provided because it determines how to decode Blocks 3 and 4.
    fn decode_blocks234(
        &mut self,
        block2: &Block2,
        block3: &Option<Block3>,
        block4: &Option<Block4>,
    ) {
        let shared = Shared::from(*block2);
        if shared.gv == GroupVariant::B && block3.is_some() {
            let pi = block3.map(|block| ProgrammeIdentifier(block.0));
            self.pi_filter.push(pi.unwrap());
        }
        self.pty_filter.push(shared.pty);
        self.tp_filter.push(shared.tp);

        if shared.gt.0 == 0 {
            if let Some(block4) = block4 {
                let idx = block2.0 & 0b11;
                let chars = block4.0.to_be_bytes();
                self.ps_filter.push_segment(idx.into(), chars);
            }
        }
    }

    fn metadata(&self) -> Metadata {
        Metadata {
            pi: self.pi_filter.mode(),
            pty: self.pty_filter.mode(),
            tp: self.tp_filter.mode(),
            ps: self.ps_filter.confirmed(),
        }
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
