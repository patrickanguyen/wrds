use crate::decoder::bitset::Bitset;
use crate::types::{RadioText, MAX_RT_SIZE};

/// Number of RadioText segment addresses
const NUM_SEGMENTS_ADDRS: usize = 16;

/// Empty RadioText Group A message
const EMPTY_RT: [u8; MAX_RT_SIZE] = [b' '; MAX_RT_SIZE];

/// Carriage return character in RadioText
///
/// This is used to indicate the end of a RadioText message for messages that are
/// for messages that require less than 16 segments addresses to transfer.
const EARLY_RETURN: u8 = b'\r';

/// Current RadioText group being decoded
///
/// The standard forbids transmitting a mixture of Group A and Group B
/// when transmitting any given message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Group {
    A,
    B,
}

#[derive(Debug)]
pub struct RtDecoder {
    buffer: [u8; MAX_RT_SIZE],
    current_group: Option<Group>,
    text_ab: Option<bool>,
    received_segments: Bitset<NUM_SEGMENTS_ADDRS>,
}

impl RtDecoder {
    pub fn new() -> Self {
        Self {
            buffer: EMPTY_RT,
            current_group: None,
            text_ab: None,
            received_segments: Bitset::default(),
        }
    }

    pub fn push_segment_a(&mut self, index: usize, chars: [u8; 4], text_ab: bool) {
        self.push_segment(index, chars, text_ab, Group::A);
    }

    pub fn push_segment_b(&mut self, index: usize, chars: [u8; 2], text_ab: bool) {
        self.push_segment(index, chars, text_ab, Group::B);
    }

    pub fn confirmed(&self) -> Option<RadioText> {
        if self.received_segments.all() {
            return Some(RadioText(self.buffer));
        }
        return None;
    }

    fn push_segment<const N: usize>(
        &mut self,
        index: usize,
        chars: [u8; N],
        text_ab: bool,
        group: Group,
    ) {
        if self.is_reset_needed(group, text_ab) {
            self.reset(group, text_ab);
        }
        let has_early_return = self.write_chars_to_buffer(index, &chars);
        self.update_received_segments(index, has_early_return);
    }

    fn write_chars_to_buffer<const N: usize>(&mut self, index: usize, chars: &[u8; N]) -> bool {
        for (char_idx, letter) in chars.iter().enumerate() {
            if *letter == EARLY_RETURN {
                return true;
            }
            let offset = N * index + char_idx;
            if offset < self.buffer.len() {
                self.buffer[offset] = *letter;
            }
        }
        false
    }

    fn update_received_segments(&mut self, index: usize, has_early_return: bool) {
        const ALL_SEGMENTS_SET: u16 = u16::MAX;
        if has_early_return {
            self.received_segments.set(ALL_SEGMENTS_SET);
        } else {
            self.received_segments
                .set_bit(index)
                .expect("Index should be always be less than 16");
        }
    }

    fn is_reset_needed(&self, group: Group, text_ab: bool) -> bool {
        self.current_group != Some(group) || self.text_ab != Some(text_ab)
    }

    fn reset(&mut self, current_group: Group, text_ab: bool) {
        self.buffer = EMPTY_RT;
        self.current_group = Some(current_group);
        self.text_ab = Some(text_ab);
        self.received_segments.reset();
    }
}
