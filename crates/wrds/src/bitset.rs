// SPDX-FileCopyrightText: 2026 Patrick Nguyen
//
// SPDX-License-Identifier: MPL-2.0

/// Bitset error
#[derive(Debug)]
pub enum Error {
    /// Out of range index
    OutofRange,
}

/// Bitset result
pub type Result<T> = core::result::Result<T, Error>;

/// Bitset represents a set of flags
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bitset<const N: usize> {
    underlying: u16,
}

const MAX_POSITION: usize = u16::BITS as usize;

impl<const N: usize> Bitset<N> {
    /// Resets bitset to none set.
    pub fn reset(&mut self) {
        self.underlying = 0;
    }

    /// Set the bit to the value
    pub fn set(&mut self, position: usize, value: bool) -> Result<()> {
        if position > N || position > MAX_POSITION {
            return Err(Error::OutofRange);
        }
        let mask = 1 << position;
        if value {
            self.underlying |= mask
        } else {
            self.underlying &= !mask
        }
        Ok(())
    }

    /// Sets the bit at position to true
    ///
    /// # Errors
    /// Returns an error if position is outside of range
    pub fn set_bit(&mut self, position: usize) -> Result<()> {
        if position > N || position > MAX_POSITION {
            return Err(Error::OutofRange);
        }
        self.underlying |= 1 << position;
        Ok(())
    }

    /// Returns true if all the bits are set
    pub fn all(&self) -> bool {
        let val: u32 = self.underlying.into();
        let all_set = (1 << N) - 1;
        all_set == val
    }

    /// Returns the underlying value of the bitset
    pub fn value(&self) -> u16 {
        self.underlying
    }

    /// Checks if bit at `position` is set
    pub fn is_set(&self, position: usize) -> bool {
        if position > N || position > MAX_POSITION {
            return false;
        }
        let mask = (self.underlying >> position) & 1;
        mask > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `set_bit()` works correctly
    #[test]
    fn test_set_bit() {
        let mut bitset = Bitset::<16>::default();
        bitset.set_bit(1).unwrap();
        const EXPECTED_VALUE: u16 = 2;
        assert_eq!(bitset.value(), EXPECTED_VALUE)
    }

    /// Verifies that `set()` works correctly
    #[test]
    fn test_set() {
        let mut bitset = Bitset::<16>::default();
        bitset.set(1, true).unwrap();
        const EXPECTED_VALUE: u16 = 2;
        assert_eq!(bitset.value(), EXPECTED_VALUE)
    }
}
