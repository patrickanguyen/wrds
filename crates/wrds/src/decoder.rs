use crate::{
    decoder::{mode_filter::ModeFilter, ps_decoder::PsDecoder, rt_decoder::RtDecoder},
    types::{Block1, Block2, Block3, Block4, GroupVariant, Message, Metadata, ProgrammeIdentifier},
    ProgrammeType, TrafficProgram,
};

use self::shared::Shared;

mod bitset;
mod mode_filter;
mod ps_decoder;
mod rt_decoder;
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
    ps_decoder: PsDecoder,
    rt_decoder: RtDecoder,
}

impl Decoder {
    /// Create new RDS decoder.
    pub fn new() -> Self {
        Decoder {
            pi_filter: ModeFilter::new(PI_FILTER_MIN).unwrap(),
            pty_filter: ModeFilter::new(PTY_FILTER_MIN).unwrap(),
            tp_filter: ModeFilter::new(TP_FILTER_MIN).unwrap(),
            ps_decoder: PsDecoder::new(),
            rt_decoder: RtDecoder::new(),
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
        self.ps_decoder.reset();
        self.rt_decoder.reset();
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

        self.handle_group_variant_b_pi(&shared, block3);
        self.pty_filter.push(shared.pty);
        self.tp_filter.push(shared.tp);

        const GROUP_TYPE0: u8 = 0;
        const GROUP_TYPE2: u8 = 2;

        match shared.gt.0 {
            GROUP_TYPE0 => {
                if let Some(block4) = block4 {
                    self.handle_ps_name(block2, block4);
                }
            }
            GROUP_TYPE2 => self.handle_radio_text(&shared, block2, block3, block4),
            _ => {}
        }
    }

    fn handle_group_variant_b_pi(&mut self, shared: &Shared, block3: &Option<Block3>) {
        if shared.gv == GroupVariant::B {
            if let Some(block3) = block3 {
                self.pi_filter.push(ProgrammeIdentifier(block3.0));
            }
        }
    }

    fn handle_ps_name(&mut self, block2: &Block2, block4: &Block4) {
        const PS_IDX_BITMASK: u16 = 0b11;
        let idx = block2.0 & PS_IDX_BITMASK;
        let chars = block4.0.to_be_bytes();
        self.ps_decoder
            .push_segment(idx.into(), chars)
            .expect("PS segment index should always be valid after bit-masking");
    }

    fn handle_radio_text(
        &mut self,
        shared: &Shared,
        block2: &Block2,
        block3: &Option<Block3>,
        block4: &Option<Block4>,
    ) {
        const RT_IDX_BITMASK: u16 = 0b1111;
        let index: usize = (block2.0 & RT_IDX_BITMASK).into();
        const TEXT_AB_BITMASK: u16 = 0x10;
        let text_ab = (block2.0 & TEXT_AB_BITMASK) > 0;
        match shared.gv {
            GroupVariant::A => {
                self.handle_radio_text_a(index, text_ab, block3, block4);
            }
            GroupVariant::B => {
                self.handle_radio_text_b(index, text_ab, block4);
            }
        }
    }

    fn handle_radio_text_a(
        &mut self,
        index: usize,
        text_ab: bool,
        block3: &Option<Block3>,
        block4: &Option<Block4>,
    ) {
        if let (Some(block3), Some(block4)) = (block3, block4) {
            let chars3 = block3.0.to_be_bytes();
            let chars4 = block4.0.to_be_bytes();
            let chars = [chars3[0], chars3[1], chars4[0], chars4[1]];
            self.rt_decoder.push_segment_a(index, chars, text_ab);
        }
    }

    fn handle_radio_text_b(&mut self, index: usize, text_ab: bool, block4: &Option<Block4>) {
        if let Some(block4) = block4 {
            let chars = block4.0.to_be_bytes();
            self.rt_decoder.push_segment_b(index, chars, text_ab);
        }
    }

    fn metadata(&self) -> Metadata {
        Metadata {
            pi: self.pi_filter.mode(),
            pty: self.pty_filter.mode(),
            tp: self.tp_filter.mode(),
            ps: self.ps_decoder.confirmed(),
            rt: self.rt_decoder.confirmed(),
        }
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
