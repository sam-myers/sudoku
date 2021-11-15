use ::std::num::NonZeroU8;

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
