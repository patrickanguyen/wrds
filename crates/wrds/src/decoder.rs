// SPDX-FileCopyrightText: 2026 Patrick Nguyen
//
// SPDX-License-Identifier: MPL-2.0

use crate::{
    decoder::{
        infoflags_filter::InfoFlagsFilters,
        mode_filter::ModeFilter,
        oda_identifier::{OdaApplication, OdaIdentifier},
        ps_decoder::PsDecoder,
        rt_decoder::RtDecoder,
    },
    types::{
        Block1, Block2, Block3, Block4, GroupType, GroupVariant, InfoFlags, Message, Metadata,
        ProgrammeIdentification, RadioTextPlusContentType, RadioTextPlusTag,
    },
    ProgrammeType, TrafficProgram,
};

use self::shared::Shared;

mod infoflags_filter;
mod mode_filter;
mod oda_identifier;
mod ps_decoder;
mod rds_charset;
mod rt_decoder;
mod shared;

const DEFAULT_FILTER_COUNT: usize = 6;
const DEFAULT_FILTER_MIN: usize = 5;

/// A decoder for Radio Data System.
#[derive(Debug)]
pub struct Decoder {
    pi_filter: ModeFilter<ProgrammeIdentification, DEFAULT_FILTER_COUNT>,
    pty_filter: ModeFilter<ProgrammeType, DEFAULT_FILTER_COUNT>,
    info_flags_filter: InfoFlagsFilters,
    ps_decoder: PsDecoder,
    rt_decoder: RtDecoder,
    oda_identifier: OdaIdentifier,
}

impl Decoder {
    /// Create new RDS decoder.
    pub fn new() -> Self {
        Decoder {
            pi_filter: ModeFilter::new(DEFAULT_FILTER_MIN).unwrap(),
            pty_filter: ModeFilter::new(DEFAULT_FILTER_MIN).unwrap(),
            info_flags_filter: InfoFlagsFilters::default(),
            ps_decoder: PsDecoder::new(),
            rt_decoder: RtDecoder::new(),
            oda_identifier: oda_identifier::OdaIdentifier::new(),
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
    ///
    /// This method should be called after tuning to a different station.
    pub fn reset(&mut self) {
        self.pi_filter.reset();
        self.pty_filter.reset();
        self.info_flags_filter.reset();
        self.ps_decoder.reset();
        self.rt_decoder.reset();
    }

    /// Decode Block 1 as the Programme Identifier (PI) if provided.
    fn decode_block1(&mut self, block1: &Option<Block1>) {
        let maybe_pi = block1.map(|block| ProgrammeIdentification(block.0));
        if let Some(pi) = maybe_pi {
            self.pi_filter.push(pi);
        }
    }

    /// Decode Blocks 2, 3, and 4.
    ///
    /// Block 2 must be provided because it determines how to decode Blocks 3 and 4.
    fn decode_blocks234(
        &mut self,
        block2: &Block2,
        maybe_block3: &Option<Block3>,
        maybe_block4: &Option<Block4>,
    ) {
        let shared = Shared::from(*block2);

        self.handle_group_variant_b_pi(&shared, maybe_block3);
        self.pty_filter.push(shared.pty);
        self.info_flags_filter.push_tp(shared.tp.0);

        const GROUP_TYPE0: GroupType = GroupType(0);
        const GROUP_TYPE2: GroupType = GroupType(2);
        const GROUP_TYPE3: GroupType = GroupType(3);
        const GROUP_TYPE15: GroupType = GroupType(15);

        match (shared.gt, shared.gv) {
            (GROUP_TYPE0, gv) => {
                self.handle_group0(gv, block2, maybe_block4);
            }
            (GROUP_TYPE2, _) => self.handle_radio_text(&shared, block2, maybe_block3, maybe_block4),
            (GROUP_TYPE3, GroupVariant::A) => {
                self.handle_oda_identification(block2, maybe_block3, maybe_block4)
            }
            (GROUP_TYPE15, GroupVariant::B) => {
                self.handle_group15b(block2, maybe_block4);
            }
            (gt, gv) if self.oda_identifier.is_registered(gt, gv) => {
                let app = self
                    .oda_identifier
                    .get_app(gt, gv)
                    .expect("The app should exist in ODA Identifier");
                self.handle_oda(app, block2, maybe_block3, maybe_block4);
            }
            _ => {}
        }
    }

    /// Handle decoding PI from Variant B RDS messages
    fn handle_group_variant_b_pi(&mut self, shared: &Shared, block3: &Option<Block3>) {
        if shared.gv == GroupVariant::B {
            if let Some(block3) = block3 {
                self.pi_filter.push(ProgrammeIdentification(block3.0));
            }
        }
    }

    /// Handles Group 0 messages.
    ///
    /// Assumes that blocks comes from correct RDS group.
    fn handle_group0(
        &mut self,
        _variant: GroupVariant,
        block2: &Block2,
        maybe_block4: &Option<Block4>,
    ) {
        self.handle_ta_ms(block2.0);

        let (decoder_control_bits, decoder_control_value) = Self::get_di(block2.0);
        self.handle_di(decoder_control_bits, decoder_control_value);

        if let Some(block4) = maybe_block4 {
            self.handle_ps_name(decoder_control_bits, block4);
        }
    }

    /// Handles Group 15B messages
    ///
    /// Assumes that the block comes from the correct RDS group.
    fn handle_group15b(&mut self, block2: &Block2, maybe_block4: &Option<Block4>) {
        self.handle_ta_ms(block2.0);
        let (decoder_control_bits, decoder_control_value) = Self::get_di(block2.0);
        self.handle_di(decoder_control_bits, decoder_control_value);

        if let Some(block4) = maybe_block4 {
            self.handle_ta_ms(block4.0);
            let (decoder_control_bits, decoder_control_value) = Self::get_di(block4.0);
            self.handle_di(decoder_control_bits, decoder_control_value);
        }
    }

    /// Handle TA and MS flags from block
    ///
    /// Assumes that the block is from the correct RDS group.
    fn handle_ta_ms(&mut self, block: u16) {
        const TA_BITMASK: u16 = 0b10000;
        let ta = block & TA_BITMASK;
        self.info_flags_filter.push_ta(ta > 0);

        const MS_BITMASK: u16 = 0b1000;
        let ms = block & MS_BITMASK;
        self.info_flags_filter.push_ms(ms > 0);
    }

    /// Get DI control bits and value from block
    ///
    /// Assumes that the block is from the correct RDS group.
    fn get_di(block: u16) -> (u16, bool) {
        const DI_CONTROL_BITMASK: u16 = 0b011;
        let decoder_control_bits = block & DI_CONTROL_BITMASK;
        const DI_VALUE_BITMASK: u16 = 0b100;
        let decode_control_value = (block & DI_VALUE_BITMASK) > 0;
        (decoder_control_bits, decode_control_value)
    }

    /// Handles decoder control bits
    ///
    /// Currently only handles stereo
    fn handle_di(&mut self, control_bits: u16, value: bool) {
        const STEREO: u16 = 0b11;
        if control_bits == STEREO {
            self.info_flags_filter.push_stereo(value);
        }
    }

    /// Handle PS message
    ///
    /// Assumes that `decoder_control_bits` is between 0 to 3, inclusive.
    fn handle_ps_name(&mut self, decoder_control_bits: u16, block4: &Block4) {
        debug_assert!(
            decoder_control_bits <= 3,
            "Decoder control bits should been bitmasked"
        );
        let chars = block4.0.to_be_bytes();
        self.ps_decoder
            .push_segment(decoder_control_bits.into(), chars)
            .expect("PS segment index should always be valid after bit-masking");
    }

    /// Handle RadioText message
    fn handle_radio_text(
        &mut self,
        shared: &Shared,
        block2: &Block2,
        maybe_block3: &Option<Block3>,
        maybe_block4: &Option<Block4>,
    ) {
        const RT_IDX_BITMASK: u16 = 0b1111;
        let index: usize = (block2.0 & RT_IDX_BITMASK).into();
        const TEXT_AB_BITMASK: u16 = 0x10;
        let text_ab = (block2.0 & TEXT_AB_BITMASK) > 0;
        match shared.gv {
            GroupVariant::A => {
                self.handle_radio_text_a(index, text_ab, maybe_block3, maybe_block4);
            }
            GroupVariant::B => {
                self.handle_radio_text_b(index, text_ab, maybe_block4);
            }
        }
    }

    /// Handle RadioText A message
    fn handle_radio_text_a(
        &mut self,
        index: usize,
        text_ab: bool,
        maybe_block3: &Option<Block3>,
        maybe_block4: &Option<Block4>,
    ) {
        if let (Some(block3), Some(block4)) = (maybe_block3, maybe_block4) {
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

    /// Handle ODA identification message
    fn handle_oda_identification(
        &mut self,
        block2: &Block2,
        maybe_block3: &Option<Block3>,
        maybe_block4: &Option<Block4>,
    ) {
        let (_block3, block4) = match maybe_block3.zip(*maybe_block4) {
            Some(v) => v,
            None => return,
        };
        let oda_app = match OdaApplication::try_from(block4.0) {
            Ok(val) => val,
            Err(_) => return,
        };
        let oda_variant = {
            const ODA_VARIANT_BITMASK: u16 = 0x1;
            GroupVariant::from(block2.0 & ODA_VARIANT_BITMASK == 1)
        };
        let oda_group = {
            const ODA_BIT_SHIFT: usize = 1;
            const ODA_GROUP_BITMASK: u16 = 0b1111;
            let value: u8 = ((block2.0 >> ODA_BIT_SHIFT) & ODA_GROUP_BITMASK)
                .try_into()
                .expect("The ODA group value should fit within 8 bits after bit-masking");
            GroupType::try_from(value).expect("The group type should be less than the maximum")
        };
        if OdaIdentifier::is_possible_oda_group(oda_group, oda_variant) {
            self.oda_identifier
                .add_new_app(oda_group, oda_variant, oda_app)
                .expect("Adding new app should not raise an error");
        }
    }

    /// Handle ODA message
    fn handle_oda(
        &mut self,
        app: OdaApplication,
        block2: &Block2,
        maybe_block3: &Option<Block3>,
        maybe_block4: &Option<Block4>,
    ) {
        // Only RT+ is supported for now
        if app != OdaApplication::RtPlus {
            return;
        }

        let (block3, block4) = match maybe_block3.zip(*maybe_block4) {
            Some(v) => v,
            None => return,
        };

        let rt_type2_first = block3.0 & 0b1;

        let tag1 = {
            let rt_content_type_1_first_part = block2.0 & 0b111;
            let bitmask_block3 = block3.0 >> 1;
            let rt_length1 = (bitmask_block3 & 0x3F) as u8;
            let bitmask_block3 = bitmask_block3 >> 6;
            let start1 = (bitmask_block3 & 0x3F) as u8;
            let bitmask_block3 = bitmask_block3 >> 6;
            let rt_content_type1: u8 = (rt_content_type_1_first_part << 3 | (bitmask_block3 & 0x7))
                .try_into()
                .unwrap();
            let tag_type1 = match RadioTextPlusContentType::try_from(rt_content_type1) {
                Ok(tag) => tag,
                Err(_) => return,
            };
            RadioTextPlusTag::new(tag_type1, start1.into(), rt_length1.into())
        };

        let tag2 = {
            let rt_length2 = block4.0 & 0x1F;
            let block4 = block4.0 >> 5;
            let start2 = (block4 & 0x3F) as u8;
            let block4 = block4 >> 6;
            let rt_content_type2: u8 = (block4 | (rt_type2_first << 5)).try_into().unwrap();
            let rt_tag2 = match RadioTextPlusContentType::try_from(rt_content_type2) {
                Ok(tag) => tag,
                Err(_) => return,
            };
            RadioTextPlusTag::new(rt_tag2, start2.into(), rt_length2.into())
        };

        self.rt_decoder.push_rt_plus_tags(tag1, tag2);
    }

    /// Returns the current metadata decoded
    pub fn metadata(&self) -> Metadata {
        let info_flags = self.info_flags_filter.info_flags();
        Metadata {
            pi: self.pi_filter.mode(),
            pty: self.pty_filter.mode(),
            tp: Some(TrafficProgram(
                info_flags.is_set(InfoFlags::TRAFFIC_PROGRAM),
            )),
            ps: self.ps_decoder.confirmed(),
            rt: self.rt_decoder.confirmed(),
            info_flags,
        }
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that:
    ///   - Decoder will do nothing if empty RDS message is decoded.
    #[test]
    fn test_empty_message() {
        let message = Message::new(None, None, None, None);
        let mut decoder = Decoder::default();
        let metadata = decoder.decode(&message);
        assert_eq!(metadata, Metadata::default())
    }

    /// Verifies that:
    ///   - Decoder will use PI from Block 1 if provided.
    #[test]
    fn test_block1_pi() {
        const EXPECTED_PI: u16 = 0x1234;

        let message = Message::new(Some(EXPECTED_PI), None, None, None);
        let mut decoder = Decoder::default();

        for _ in 0..10 {
            let _ = decoder.decode(&message);
        }

        let metadata = decoder.decode(&message);
        assert_eq!(metadata.pi(), Some(ProgrammeIdentification(EXPECTED_PI)))
    }

    /// Verifies that:
    ///   - Decoder will use the PI from Block 3 if Block 1 is not provided and if Group Variant is Type B.
    #[test]
    fn test_block3_pi() {
        const EXPECTED_PI: u16 = 0x5678;
        const BLOCK2: u16 = 0xBEEF;

        let message = Message::new(None, Some(BLOCK2), Some(EXPECTED_PI), None);
        let mut decoder = Decoder::default();

        for _ in 0..10 {
            let _ = decoder.decode(&message);
        }
        let metadata = decoder.decode(&message);
        assert_eq!(metadata.pi(), Some(ProgrammeIdentification(EXPECTED_PI)));
    }

    /// Verifies that:
    ///   - [`Decoder::reset()`] resets all of the metadata to None
    #[test]
    fn test_reset() {
        const EXPECTED_PI: u16 = 0x5678;
        const BLOCK2: u16 = 0xBEEF;

        let message = Message::new(None, Some(BLOCK2), Some(EXPECTED_PI), None);
        let mut decoder = Decoder::default();

        for _ in 0..10 {
            let _ = decoder.decode(&message);
        }
        decoder.reset();
        let metadata = decoder.metadata();
        assert_eq!(metadata.pi, None)
    }

    /// Verifies that:
    ///   - Decoder is able to successfully decode messages and retrieve PI, PTY,
    ///     TP, RT, PS from data received.
    #[test]
    fn test_decode_real_data() {
        let raw_messages = [
            [0x137A, 0x20E4, 0x6F20, 0x4E6F],
            [0x137A, 0x20E3, 0x3073, 0x2074],
            [0x137A, 0x00E3, 0xE0CD, 0x3073],
            [0x137A, 0x20E3, 0x3073, 0x2074],
            [0x137A, 0x00E1, 0xE0CD, 0x666D],
            [0x137A, 0x20E0, 0x3130, 0x342E],
            [0x137A, 0x00E3, 0xE0CD, 0x3073],
            [0x137A, 0x20E1, 0x3320, 0x4D59],
            [0x137A, 0x20E5, 0x7720, 0x0D20],
            [0x137A, 0x20E0, 0x3130, 0x342E],
            [0x137A, 0x20E2, 0x666D, 0x2039],
            [0x137A, 0x00E0, 0xE0CD, 0x3130],
            [0x137A, 0x00E2, 0xE0CD, 0x3320],
        ];

        let mut decoder = Decoder::new();

        for raw_message in raw_messages {
            let [block1, block2, block3, block4] = raw_message;
            let blocks = Message::new(Some(block1), Some(block2), Some(block3), Some(block4));
            decoder.decode(&blocks);
        }

        let metadata = decoder.metadata();

        assert_eq!(metadata.pi, Some(ProgrammeIdentification(0x137A)));

        assert_eq!(metadata.pty, Some(ProgrammeType(7)));

        assert_eq!(metadata.tp, Some(TrafficProgram(false)));

        let rt = metadata.rt.unwrap();
        assert_eq!(rt.as_str(), "104.3 MYfm 90s to Now ");

        // Note that this is correct based off data received
        let ps = metadata.ps.unwrap();
        assert_eq!(ps.as_str(), "10fm3 0s");
    }

    /// Verifies that:
    ///   - Decoder is able to properly decode RT+ data
    #[test]
    fn test_decode_rt_plus() {
        let raw_messages = [
            [0x137A, 0x00E1, 0xE0CD, 0x342E],
            [0x137A, 0x20E0, 0x3130, 0x342E],
            [0x137A, 0x20E1, 0x3320, 0x4D59],
            [0x137A, 0x20E2, 0x666D, 0x2042],
            [0x137A, 0x00E0, 0xE0CD, 0x3130],
            [0x137A, 0x20E8, 0x6C6C, 0x6965],
            [0x137A, 0x00E2, 0xE0CD, 0x3320],
            [0x137A, 0x20EB, 0x0D20, 0x2020],
            [0x137A, 0x20E0, 0x3130, 0x342E],
            [0x137A, 0x20E4, 0x204F, 0x6620],
            [0x137A, 0x00E0, 0xE0CD, 0x4D59],
            [0x137A, 0x20E5, 0x4120, 0x4665],
            [0x137A, 0x30F8, 0x0000, 0x4BD7],
            [0x137A, 0x00E2, 0xE0CD, 0x2020],
            [0x137A, 0x20EA, 0x6973, 0x6820],
            [0x137A, 0x20E2, 0x666D, 0x2042],
            [0x137A, 0x20E3, 0x6972, 0x6473],
            [0x137A, 0x20E8, 0x6C6C, 0x6965],
            [0x137A, 0x20E2, 0x666D, 0x2042],
            [0x137A, 0x20E6, 0x6174, 0x6865],
            [0x137A, 0xC0E8, 0x8F18, 0x0971],
            [0x137A, 0x20EB, 0x0D20, 0x2020],
            [0x137A, 0x20E0, 0x3130, 0x342E],
            [0x137A, 0x20E1, 0x3320, 0x4D59],
            [0x137A, 0x20E8, 0x6C6C, 0x6965],
            [0x137A, 0x20E9, 0x2045, 0x696C],
            [0x137A, 0x20E2, 0x666D, 0x2042],
            [0x137A, 0x20E6, 0x6174, 0x6865],
            [0x137A, 0x20E7, 0x7220, 0x4269],
            [0x137A, 0x00E3, 0xE0CD, 0x4F66],
            [0x137A, 0x20E0, 0x3130, 0x342E],
            [0x137A, 0x20E1, 0x3320, 0x4D59],
            [0x137A, 0x00E1, 0xE0CD, 0x2020],
            [0x137A, 0x20E4, 0x204F, 0x6620],
            [0x137A, 0x00E3, 0xE0CD, 0x2020],
            [0x137A, 0x20EA, 0x6973, 0x6820],
            [0x137A, 0x20EB, 0x0D20, 0x2020],
            [0x137A, 0x20E0, 0x3130, 0x342E],
        ];

        let mut decoder = Decoder::new();

        for raw_message in raw_messages {
            let [block1, block2, block3, block4] = raw_message;
            let blocks = Message::new(Some(block1), Some(block2), Some(block3), Some(block4));
            decoder.decode(&blocks);
        }

        let metadata = decoder.metadata();

        assert_eq!(metadata.pi, Some(ProgrammeIdentification(0x137A)));

        assert_eq!(metadata.pty, Some(ProgrammeType(7)));

        assert_eq!(metadata.tp, Some(TrafficProgram(false)));

        let rt = metadata.rt.unwrap();
        assert_eq!(rt.as_str(), "104.3 MYfm Birds Of A Feather Billie Eilish ");

        let rt_plus = rt.rt_plus();
        assert_eq!(
            rt_plus,
            [
                RadioTextPlusTag::new(RadioTextPlusContentType::Artist, 30, 12),
                RadioTextPlusTag::new(RadioTextPlusContentType::Title, 11, 17)
            ]
        )
    }
}
