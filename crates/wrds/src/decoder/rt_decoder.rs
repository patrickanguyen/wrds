use crate::types::RadioText;

use crate::{
    decoder::bitset::Bitset,
    types::{MAX_RT_SIZE},
};

/// Empty RadioText Group A message
const EMPTY_RT: [u8; MAX_RT_SIZE] = [b' '; MAX_RT_SIZE];

/// Carriage return character in RadioText
///
/// This is used to indicate the end of a RadioText message for messages that are
/// for messages that require less than 16 segments addresses to transfer.
const EARLY_RETURN: u8 = b'\r';

/// Space character in RadioText
///
/// Used to replace invalid characters and serve as padding.
const SPACE: u8 = b' ';

/// Number of segments in RadioText
const NUM_SEGMENTS: usize = 16;

/// Size of segment in RadioText for Group A
const SEGMENT_SIZE_A: usize = 4;

/// Size of segment in RadioText for Group B
const SEGMENT_SIZE_B: usize = 2;

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
    received_segments: Bitset<NUM_SEGMENTS>,
    early_idx: Option<usize>,
}

impl RtDecoder {
    pub fn new() -> Self {
        Self {
            buffer: EMPTY_RT,
            current_group: None,
            text_ab: None,
            received_segments: Bitset::default(),
            early_idx: None,
        }
    }

    pub fn push_segment_a(&mut self, index: usize, chars: [u8; 4], text_ab: bool) {
        self.push_segment(index, chars, text_ab, Group::A);
    }

    pub fn push_segment_b(&mut self, index: usize, chars: [u8; 2], text_ab: bool) {
        self.push_segment(index, chars, text_ab, Group::B);
    }

    pub fn confirmed(&self) -> Option<RadioText> {
        let segment_size = match self.current_group {
            Some(Group::A) => SEGMENT_SIZE_A,
            Some(Group::B) => SEGMENT_SIZE_B,
            None => return None, // No group set, cannot confirm
        };
        let length = match (self.early_idx, self.current_group) {
            (Some(early), _) => early,
            (None, Some(Group::A)) => MAX_RT_SIZE,
            (None, Some(Group::B)) => MAX_RT_SIZE / 2,
            (None, None) => return None, // No group set, cannot confirm
        };
        // Check if all required segments are received
        let required_segments = length / segment_size;
        let required_bitmask: u32 = (1 << required_segments) - 1;
        let received_bitmask: u32 = self.received_segments.value().into();
        if (received_bitmask & required_bitmask) == required_bitmask {
            let vec = heapless::Vec::from_slice(&self.buffer[..length])
                .expect("self.buffer should always fit in heapless::Vec");
            let rt_string = heapless::String::from_utf8(vec)
                .expect("self.buffer should always contain valid UTF-8");
            return Some(RadioText(rt_string));
        }
        None
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
        self.write_chars_to_buffer(index, &chars);
    }

    fn write_chars_to_buffer<const N: usize>(&mut self, segment_idx: usize, chars: &[u8; N]) {
        for (char_idx, letter) in chars.iter().enumerate() {
            let letter_idx = N * segment_idx + char_idx;
            let rt_char = if Self::is_rt_character_valid(*letter) {
                if Some(letter_idx) == self.early_idx {
                    self.early_idx = None; // Reset early index if we are writing a valid character
                }
                *letter
            } else if *letter == EARLY_RETURN {
                self.early_idx = Some(letter_idx);
                *letter
            } else {
                SPACE // Replace invalid characters with space
            };
            debug_assert!(letter_idx < MAX_RT_SIZE, "Index should always be within bounds");
            self.buffer[letter_idx] = rt_char;
        }
        self.received_segments
            .set_bit(segment_idx)
            .expect("Segment index should always be less than NUM_SEGMENTS");
    }

    fn is_reset_needed(&self, _group: Group, text_ab: bool) -> bool {
        self.text_ab != Some(text_ab)
    }

    fn reset(&mut self, current_group: Group, text_ab: bool) {
        self.buffer = EMPTY_RT;
        self.current_group = Some(current_group);
        self.text_ab = Some(text_ab);
        self.received_segments.reset();
        self.early_idx = None;
    }

    fn is_rt_character_valid(c: u8) -> bool {
        c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c == SPACE
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_decoder_is_empty() {
        let decoder = RtDecoder::new();
        assert_eq!(decoder.current_group, None);
        assert_eq!(decoder.text_ab, None);
        assert_eq!(decoder.early_idx, None);
        assert_eq!(decoder.buffer, EMPTY_RT);
        assert_eq!(decoder.confirmed(), None);
    }

    #[test]
    fn test_push_segment_a_and_confirmed_full() {
        let mut decoder = RtDecoder::new();
        let text_ab = true;
        let chars = [b'T', b'E', b'S', b'T'];
        for i in 0..NUM_SEGMENTS {
            decoder.push_segment_a(i, chars, text_ab);
        }
        let expected_text = String::from("TEST".repeat(NUM_SEGMENTS));
        let expected = RadioText(
            heapless::String::from_iter(expected_text.chars())
        );
        assert_eq!(decoder.confirmed(), Some(expected));
    }

    #[test]
    fn test_push_segment_b_and_confirmed_full() {
        let mut decoder = RtDecoder::new();
        let text_ab = false;
        let chars = [b'O', b'K'];
        for i in 0..NUM_SEGMENTS {
            decoder.push_segment_b(i, chars, text_ab);
        }
        let expected_text = String::from("OK".repeat(NUM_SEGMENTS));
        let expected = RadioText(
            heapless::String::from_iter(expected_text.chars())
        );
        assert_eq!(decoder.confirmed(), Some(expected));
    }

    #[test]
    fn test_early_return_truncates_text() {
        let mut decoder = RtDecoder::new();
        let text_ab = true;
        // Fill first 3 segments, then insert EARLY_RETURN in the 4th
        for i in 0..3 {
            decoder.push_segment_a(i, [b'A', b'B', b'C', b'D'], text_ab);
        }
        decoder.push_segment_a(3, [b'E', b'F', EARLY_RETURN, b'H'], text_ab);
        // Fill remaining segments (should be ignored)
        for i in 4..NUM_SEGMENTS {
            decoder.push_segment_a(i, [b'X', b'X', b'X', b'X'], text_ab);
        }
        let expected_text = String::from("ABCDABCDABCDEF"); // Up to EARLY_RETURN
        // Confirmed should only include up to the early return
        let expected = RadioText(
            heapless::String::from_iter(expected_text.chars() /* up to index 11 */)
        );
        // Confirmed should only be available if all segments up to early_idx are received
        assert_eq!(decoder.confirmed(), Some(expected));
    }

    #[test]
    fn test_invalid_characters_are_replaced_with_space() {
        let mut decoder = RtDecoder::new();
        let text_ab = false;
        let invalid = [0xFF, 0x80, b'A', b'B'];
        decoder.push_segment_a(0, invalid, text_ab);
        let mut expected = [b' '; MAX_RT_SIZE];
        expected[0] = b' ';
        expected[1] = b' ';
        expected[2] = b'A';
        expected[3] = b'B';
        assert_eq!(&decoder.buffer[..4], &expected[..4]);
    }

    #[test]
    fn test_reset_on_text_ab_change() {
        let mut decoder = RtDecoder::new();
        decoder.push_segment_a(0, [b'A', b'B', b'C', b'D'], true);
        assert_eq!(decoder.text_ab, Some(true));
        decoder.push_segment_a(1, [b'E', b'F', b'G', b'H'], false);
        assert_eq!(decoder.text_ab, Some(false));
        // After reset, only segment 1 should be set
        assert_eq!(&decoder.buffer[..4], &[b' '; 4]);
        assert_eq!(&decoder.buffer[4..8], &[b'E', b'F', b'G', b'H']);
    }

    #[test]
    fn test_confirmed_none_if_not_all_segments_received() {
        let mut decoder = RtDecoder::new();
        decoder.push_segment_a(0, [b'A', b'B', b'C', b'D'], true);
        // Only one segment, not enough for confirmation
        assert_eq!(decoder.confirmed(), None);
    }
}
