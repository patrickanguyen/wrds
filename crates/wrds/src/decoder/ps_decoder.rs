use crate::decoder::bitset::Bitset;

/// Maximum of PS
const PS_SIZE: usize = 8;

/// Empty PS
const EMPTY_PS: [u8; PS_SIZE] = [b' '; PS_SIZE];

/// Decoder for Programme Service Name (PS)
#[derive(Debug)]
pub struct PsDecoder {
    segments: [u8; PS_SIZE],
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
    /// Resets the segments if all of the segments were already set
    pub fn push_segment(&mut self, index: usize, chars: [u8; 2]) {
        if self.is_chars_set.all() {
            self.segments = EMPTY_PS;
            self.is_chars_set.reset();
        }
        if index < self.segments.len() {
            self.segments[2 * index] = chars[0];
            self.segments[(2 * index) + 1] = chars[1];
            self.is_chars_set.set_bit(index).unwrap();
        }
    }

    /// Confirms if complete PS has been ready.
    ///
    /// - If ready, returns PS represented as bytes.
    /// - If not, returns `None`.
    pub fn confirmed(&self) -> Option<[u8; 8]> {
        if !self.is_chars_set.all() {
            return None;
        }
        Some(self.segments)
    }
}
