use crate::decoder::mode_filter::{Error, ModeFilter};

const NUM_SEGMENTS: usize = 4;
const SEGMENT_SIZE: usize = 2;

#[derive(Debug)]
pub struct PsFilter<const N: usize> {
    segments: [ModeFilter<[u8; SEGMENT_SIZE], N>; NUM_SEGMENTS],
}

impl<const N: usize> PsFilter<N> {
    pub fn new(min_count: usize) -> Result<Self, Error> {
        if min_count > N {
            return Err(Error::MinCountExceedsSize { min_count, size: N });
        }

        Ok(Self {
            segments: [
                ModeFilter::new(min_count).unwrap(),
                ModeFilter::new(min_count).unwrap(),
                ModeFilter::new(min_count).unwrap(),
                ModeFilter::new(min_count).unwrap(),
            ],
        })
    }

    pub fn push_segment(&mut self, index: usize, chars: [u8; 2]) {
        if index < self.segments.len() {
            self.segments[index].push(chars);
        }
    }

    pub fn confirmed(&self) -> Option<[u8; 8]> {
        if self.segments.iter().any(|segment| segment.mode().is_none()) {
            return None;
        }
        let mut out = [b' '; 8];
        for (i, segment) in self.segments.iter().enumerate() {
            let [char1, char2] = segment.mode().unwrap();
            out[2 * i] = char1;
            out[2 * i + 1] = char2;
        }
        Some(out)
    }
}
