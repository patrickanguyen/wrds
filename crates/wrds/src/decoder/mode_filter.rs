use core::fmt;

/// ModeFilter Error
#[derive(Debug)]
pub enum Error {
    /// ModeFilter `min_count` exceeds `size`.
    MinCountExceedsSize { min_count: usize, size: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MinCountExceedsSize { min_count, size } => {
                write!(f, "Min count `{min_count}` exceeds size `{size}`")
            }
        }
    }
}

/// [`ModeFilter`] is a filter that finds the mode given `N` samples.
///
/// It will return `None` until `N` samples are recorded and the mode has exceeded
/// `min_count`.
#[derive(Debug, Clone)]
pub struct ModeFilter<T, const N: usize> {
    buf: [Option<T>; N],
    index: usize,
    min_count: usize,
}

impl<T, const N: usize> ModeFilter<T, N>
where
    T: PartialEq + Copy,
{
    /// Create new instance of [`ModeFilter`]
    ///
    /// # Errors
    /// Returns an error if `min_count` exceeds `N`, the maximum sample count.
    pub fn new(min_count: usize) -> Result<Self, Error> {
        if min_count > N {
            return Err(Error::MinCountExceedsSize { min_count, size: N });
        }

        Ok(Self {
            buf: [None; N],
            index: 0,
            min_count,
        })
    }

    /// Push new sample to [`ModeFilter`]
    ///
    /// This will override the oldest value if the total number of samples exceeds `N`.
    pub fn push(&mut self, value: T) {
        self.buf[self.index] = Some(value);
        self.index = (self.index + 1) % N;
    }

    /// Returns the statistical mode of the samples.
    ///
    /// Will return `None` if the following scenarios
    /// - Number of samples is less than `N`
    /// - The mode count is less than the `min_count`
    pub fn mode(&self) -> Option<T> {
        if self.buf.iter().any(|x| x.is_none()) {
            return None;
        }

        let mut best_val = self.buf[0].unwrap();
        let mut best_count = 0;

        for &candidate in &self.buf.map(|x| x.unwrap()) {
            let mut count = 0;
            for &v in &self.buf.map(|x| x.unwrap()) {
                if v == candidate {
                    count += 1;
                }
            }
            if count > best_count {
                best_count = count;
                best_val = candidate;
            }
        }

        if best_count >= self.min_count {
            Some(best_val)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.buf = [None; N];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut mode_filter = ModeFilter::<u16, 10>::new(6).unwrap();
        mode_filter.push(0x123);
        assert_eq!(mode_filter.mode(), None)
    }

    #[test]
    fn test2() {
        let mut mode_filter = ModeFilter::<u16, 1>::new(1).unwrap();
        mode_filter.push(0x123);
        mode_filter.push(0x123);
        assert_eq!(mode_filter.mode(), Some(0x123))
    }
}
