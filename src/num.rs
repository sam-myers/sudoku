#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Num {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

impl Num {
    pub fn to_int(&self) -> u8 {
        match self {
            Num::One   => 1,
            Num::Two   => 2,
            Num::Three => 3,
            Num::Four  => 4,
            Num::Five  => 5,
            Num::Six   => 6,
            Num::Seven => 7,
            Num::Eight => 8,
            Num::Nine  => 9,
        }
    }
}
