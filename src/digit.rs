use ::std::num::NonZeroU8;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Digit(NonZeroU8);

impl Digit {
    /// Construct a new `Num`.
    ///
    /// # Panics
    /// Panics if digit is not allowed
    pub fn new(digit: u8) -> Self {
        Self::from_unverified(digit).unwrap()
    }

    /// Construct a new `Num`.
    pub fn from_unverified(digit: u8) -> Option<Self> {
        if digit < 1 || digit > 9 {
            return None;
        }
        NonZeroU8::new(digit).map(Digit)
    }

    pub fn to_int(self) -> u8 {
        self.0.get()
    }

    #[allow(dead_code)]
    pub fn from_int(i: u8) -> Option<Digit> {
        Digit::from_unverified(i)
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.get())
    }
}

#[cfg(test)]
mod tests {
    use crate::digit::Digit;

    #[test]
    fn test_valid_digits() {
        for i in 1..10 {
            Digit::new(i);
        }
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Digit::new(1)), "1");
    }
}
