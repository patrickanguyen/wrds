use crate::decoder::{bitset::Bitset, rds_charset::to_basic_rds_char};
use crate::types::{
    RadioText, RadioTextPlusList, RadioTextPlusTag, RadioTextString, MAX_RT_LENGTH,
};

/// Empty RadioText Group A message
const EMPTY_RT: [char; MAX_RT_LENGTH] = [' '; MAX_RT_LENGTH];

/// Carriage return character in RadioText
///
/// This is used to indicate the end of a RadioText message for messages that are
/// for messages that require less than 16 segments addresses to transfer.
const EARLY_RETURN: char = '\r';

/// Space character in RadioText
///
/// Used to replace invalid characters and serve as padding.
const SPACE: char = ' ';

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
    buffer: [char; MAX_RT_LENGTH],
    current_group: Option<Group>,
    text_ab: Option<bool>,
    received_segments: Bitset<NUM_SEGMENTS>,
    early_idx: Option<usize>,
    rt_tag1: Option<RadioTextPlusTag>,
    rt_tag2: Option<RadioTextPlusTag>,
}

impl RtDecoder {
    pub fn new() -> Self {
        Self {
            buffer: EMPTY_RT,
            current_group: None,
            text_ab: None,
            received_segments: Bitset::default(),
            early_idx: None,
            rt_tag1: None,
            rt_tag2: None,
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
            (None, Some(Group::A)) => MAX_RT_LENGTH,
            (None, Some(Group::B)) => MAX_RT_LENGTH / 2,
            (None, None) => return None, // No group set, cannot confirm
        };
        // Check if all required segments are received
        let required_segments = length / segment_size;
        let required_bitmask: u32 = (1 << required_segments) - 1;
        let received_bitmask: u32 = {
            let value: u32 = self.received_segments.value().into();
            value & required_bitmask
        };
        if received_bitmask == required_bitmask {
            let rt_string = RadioTextString::from_iter(&self.buffer[..length]);
            let rt_plus = {
                if self.rt_tag1.is_some() && self.rt_tag2.is_some() {
                    RadioTextPlusList::from_array([self.rt_tag1.unwrap(), self.rt_tag2.unwrap()])
                } else {
                    RadioTextPlusList::new()
                }
            };
            return Some(RadioText::new(rt_string, rt_plus));
        }
        None
    }

    pub fn push_rt_plus_tags(&mut self, tag1: RadioTextPlusTag, tag2: RadioTextPlusTag) {
        self.rt_tag1 = Some(tag1);
        self.rt_tag2 = Some(tag2);
    }

    fn push_segment<const N: usize>(
        &mut self,
        index: usize,
        chars: [u8; N],
        text_ab: bool,
        group: Group,
    ) {
        if self.is_reset_needed(group, text_ab) {
            self.internal_reset(Some(group), Some(text_ab));
        }
        self.write_chars_to_buffer(index, &chars);
    }

    fn write_chars_to_buffer<const N: usize>(&mut self, segment_idx: usize, chars: &[u8; N]) {
        for (char_idx, letter) in chars.iter().enumerate() {
            let letter_idx = N * segment_idx + char_idx;
            let rt_char = to_basic_rds_char(*letter).unwrap_or(SPACE);
            if rt_char == EARLY_RETURN {
                self.early_idx = Some(letter_idx);
            } else if Some(letter_idx) == self.early_idx {
                self.early_idx = None;
            }

            debug_assert!(
                letter_idx < MAX_RT_LENGTH,
                "Index should always be within bounds"
            );
            self.buffer[letter_idx] = rt_char;
        }
        self.received_segments
            .set_bit(segment_idx)
            .expect("Segment index should always be less than NUM_SEGMENTS");
    }

    fn is_reset_needed(&self, _group: Group, text_ab: bool) -> bool {
        self.text_ab != Some(text_ab)
    }

    pub fn reset(&mut self) {
        self.internal_reset(None, None);
        self.current_group = None;
        self.text_ab = None;
        self.rt_tag1 = None;
        self.rt_tag2 = None;
    }

    fn internal_reset(&mut self, current_group: Option<Group>, text_ab: Option<bool>) {
        self.buffer = EMPTY_RT;
        self.current_group = current_group;
        self.text_ab = text_ab;
        self.received_segments.reset();
        self.early_idx = None;
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
        let expected = RadioText::new(
            RadioTextString::from_iter(expected_text.chars()),
            RadioTextPlusList::new(),
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
        let expected = RadioText::new(
            RadioTextString::from_iter(expected_text.chars()),
            RadioTextPlusList::new(),
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
        decoder.push_segment_a(3, [b'E', b'F', b'\r', b'H'], text_ab);
        // Fill remaining segments (should be ignored)
        for i in 4..NUM_SEGMENTS {
            decoder.push_segment_a(i, [b'X', b'X', b'X', b'X'], text_ab);
        }
        let expected_text = String::from("ABCDABCDABCDEF"); // Up to EARLY_RETURN
                                                            // Confirmed should only include up to the early return
        let expected = RadioText::new(
            RadioTextString::from_iter(expected_text.chars()),
            RadioTextPlusList::new(),
        );
        // Confirmed should only be available if all segments up to early_idx are received
        assert_eq!(decoder.confirmed(), Some(expected));
    }

    #[test]
    fn test_invalid_characters_are_replaced_with_space() {
        let mut decoder = RtDecoder::new();
        let text_ab = false;
        let invalid = [0xFF, 0x02, b'A', b'B'];
        decoder.push_segment_a(0, invalid, text_ab);
        let mut expected = [' '; MAX_RT_LENGTH];
        expected[0] = ' ';
        expected[1] = ' ';
        expected[2] = 'A';
        expected[3] = 'B';
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
        assert_eq!(&decoder.buffer[..4], &[' '; 4]);
        assert_eq!(&decoder.buffer[4..8], &['E', 'F', 'G', 'H']);
    }

    #[test]
    fn test_confirmed_none_if_not_all_segments_received() {
        let mut decoder = RtDecoder::new();
        decoder.push_segment_a(0, [b'A', b'B', b'C', b'D'], true);
        // Only one segment, not enough for confirmation
        assert_eq!(decoder.confirmed(), None);
    }

    #[test]
    fn test_reset() {
        let mut decoder = RtDecoder::new();
        decoder.push_segment_a(0, [b'A', b'B', b'C', b'D'], true);
        decoder.reset();
        assert_eq!(decoder.current_group, None);
        assert_eq!(decoder.text_ab, None);
        assert_eq!(decoder.early_idx, None);
        assert_eq!(decoder.buffer, EMPTY_RT);
        assert_eq!(decoder.confirmed(), None);
    }

    #[test]
    fn test_override_early_return() {
        let mut decoder = RtDecoder::new();
        let text_ab = true;
        decoder.push_segment_a(0, [b'A', b'B', b'\r', b'D'], text_ab);
        assert_eq!(decoder.early_idx, Some(2));
        assert_eq!(
            decoder.confirmed(),
            Some(RadioText::new(
                RadioTextString::from_iter("AB".chars()),
                RadioTextPlusList::new()
            ))
        );
        // Override EARLY_RETURN with valid character
        decoder.push_segment_a(0, [b'A', b'B', b'C', b'D'], text_ab);
        assert_eq!(decoder.early_idx, None);
        let expected = ['A', 'B', 'C', 'D'];
        assert_eq!(&decoder.buffer[..4], &expected);
    }

    #[test]
    fn test_non_ascii_full_string() {
        let mut decoder = RtDecoder::new();
        for i in 0..NUM_SEGMENTS {
            decoder.push_segment_a(i, [0xAE; 4], true);
        }
        let expected = {
            let buffer = ['→'; MAX_RT_LENGTH];
            String::from_iter(buffer.iter())
        };
        assert_eq!(
            decoder.confirmed(),
            Some(RadioText::new(
                RadioTextString::from_iter(expected.chars()),
                RadioTextPlusList::new()
            ))
        );
    }
}
