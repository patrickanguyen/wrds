use crate::{
    decoder::{bitset::Bitset, rds_charset::to_basic_rds_char},
    types::{ProgrammeServiceName, ProgrammeServiceNameString},
};

/// Maximum of PS
const PS_SIZE: usize = 8;

/// Empty PS
const EMPTY_PS: [char; PS_SIZE] = [' '; PS_SIZE];

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum PsDecoderError {
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
}

pub type Result<T> = core::result::Result<T, PsDecoderError>;

/// Decoder for Programme Service Name (PS)
#[derive(Debug)]
pub struct PsDecoder {
    segments: [char; PS_SIZE],
    is_chars_set: Bitset<4>,
}

impl PsDecoder {
    /// Creates new PsDecoder
    pub fn new() -> Self {
        Self {
            segments: EMPTY_PS,
            is_chars_set: Bitset::default(),
        }
    }

    /// Push new PS segment
    ///
    /// Resets the segments if different segment is pushed when all segments are already set.
    pub fn push_segment(&mut self, index: usize, segment_bytes: [u8; 2]) -> Result<()> {
        if index >= (self.segments.len() / 2) {
            return Err(PsDecoderError::IndexOutOfBounds(index));
        }

        let current_index1 = 2 * index;
        let current_index2 = (2 * index) + 1;

        let char1 = to_basic_rds_char(segment_bytes[0]).unwrap_or(' ');
        let char2 = to_basic_rds_char(segment_bytes[1]).unwrap_or(' ');

        let incoming = [char1, char2];
        let current = &self.segments[current_index1..=current_index2];

        if self.is_chars_set.all() && current != incoming {
            self.segments = EMPTY_PS;
            self.is_chars_set.reset();
        }

        self.segments[current_index1] = char1;
        self.segments[current_index2] = char2;
        self.is_chars_set
            .set_bit(index)
            .expect("The index should always be valid");
        Ok(())
    }

    /// Confirms if complete PS has been ready.
    ///
    /// - If ready, returns PS represented as bytes.
    /// - If not, returns `None`.
    pub fn confirmed(&self) -> Option<ProgrammeServiceName> {
        if !self.is_chars_set.all() {
            return None;
        }
        let ps = ProgrammeServiceNameString::from_iter(self.segments.iter());
        Some(ProgrammeServiceName::new(ps))
    }

    pub fn reset(&mut self) {
        self.segments = EMPTY_PS;
        self.is_chars_set.reset();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_decoder_is_empty() {
        let decoder = PsDecoder::new();
        assert_eq!(decoder.segments, EMPTY_PS);
        assert!(!decoder.is_chars_set.all());
        assert_eq!(decoder.confirmed(), None);
    }

    #[test]
    fn test_push_segment_sets_segments() {
        let mut decoder = PsDecoder::new();
        decoder.push_segment(0, [b'A', b'B']).unwrap();
        assert_eq!(decoder.segments[0], 'A');
        assert_eq!(decoder.segments[1], 'B');
        assert!(!decoder.is_chars_set.all());
        assert_eq!(decoder.confirmed(), None);
    }

    #[test]
    fn test_confirmed_returns_some_when_complete() {
        let mut decoder = PsDecoder::new();
        let chars = [[b'A', b'B'], [b'C', b'D'], [b'E', b'F'], [b'G', b'H']];
        for (i, pair) in chars.iter().enumerate() {
            decoder.push_segment(i, *pair).unwrap();
        }
        assert!(decoder.is_chars_set.all());
        let expected = {
            let ps = String::from("ABCDEFGH");
            ProgrammeServiceName::new(ProgrammeServiceNameString::from_iter(ps.chars()))
        };
        assert_eq!(decoder.confirmed(), Some(expected));
    }

    #[test]
    fn test_push_segment_resets_when_full() {
        let mut decoder = PsDecoder::new();
        for i in 0..4 {
            decoder.push_segment(i, [b'X', b'Y']).unwrap();
        }
        assert!(decoder.is_chars_set.all());
        // Next push should reset
        decoder.push_segment(0, [b'A', b'B']).unwrap();
        assert_eq!(decoder.segments[0], 'A');
        assert_eq!(decoder.segments[1], 'B');
        assert!(!decoder.is_chars_set.all());
        assert_eq!(decoder.confirmed(), None);
    }

    #[test]
    fn test_reset_clears_segments_and_bits() {
        let mut decoder = PsDecoder::new();
        for i in 0..4 {
            decoder.push_segment(i, [b'X', b'Y']).unwrap();
        }
        decoder.reset();
        assert_eq!(decoder.segments, EMPTY_PS);
        assert!(!decoder.is_chars_set.all());
        assert_eq!(decoder.confirmed(), None);
    }

    #[test]
    fn test_push_segment_out_of_bounds_does_nothing() {
        let mut decoder = PsDecoder::new();
        assert_eq!(
            decoder.push_segment(4, [b'A', b'B']),
            Err(PsDecoderError::IndexOutOfBounds(4))
        );
    }

    #[test]
    fn test_push_segment_does_not_reset() {
        let mut decoder = PsDecoder::new();
        let chars = [[b'A', b'B'], [b'C', b'D'], [b'E', b'F'], [b'G', b'H']];
        for (i, pair) in chars.iter().enumerate() {
            decoder.push_segment(i, *pair).unwrap();
        }
        assert!(decoder.is_chars_set.all());
        // Next push should not reset
        decoder.push_segment(0, [b'A', b'B']).unwrap();
        let expected = {
            let ps = String::from("ABCDEFGH");
            ProgrammeServiceName::new(ProgrammeServiceNameString::from_iter(ps.chars()))
        };
        assert_eq!(decoder.confirmed(), Some(expected));
    }

    #[test]
    fn test_push_invalid_character() {
        let mut decoder = PsDecoder::new();
        assert_eq!(decoder.push_segment(0, [0x1F, b'B']), Ok(()));
        assert_eq!(decoder.push_segment(1, [b'A', 0x01]), Ok(()));
        assert_eq!(decoder.segments[0], ' ');
        assert_eq!(decoder.segments[3], ' ');
    }

    #[test]
    fn test_non_ascii_full_string() {
        let mut decoder = PsDecoder::new();
        for i in 0..(decoder.segments.len() / 2) {
            let _ = decoder.push_segment(i, [0xAE; 2]);
        }
        let expected = {
            let buffer = ['→'; PS_SIZE];
            let ps = ProgrammeServiceNameString::from_iter(buffer.iter());
            ProgrammeServiceName::new(ps)
        };
        assert_eq!(decoder.confirmed(), Some(expected));
    }
}
