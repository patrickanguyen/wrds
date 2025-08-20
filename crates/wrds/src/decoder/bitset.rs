/// Bitset error
#[derive(Debug)]
pub enum Error {
    /// Out of range index
    OutofRange,
}

/// Bitset result
pub type Result<T> = core::result::Result<T, Error>;

/// Bitset represents a set of flags
#[derive(Debug, Default)]
pub struct Bitset<const N: usize> {
    underlying: u16,
}

const MAX_POSITION: usize = u16::BITS as usize;

impl<const N: usize> Bitset<N> {
    /// Resets bitset to none set.
    pub fn reset(&mut self) {
        self.underlying = 0;
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
}
