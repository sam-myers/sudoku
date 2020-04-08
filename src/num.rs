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

    #[allow(dead_code)]
    pub fn from_int(i: u8) -> Option<Num> {
        match i {
            1 => Some(Num::One),
            2 => Some(Num::Two),
            3 => Some(Num::Three),
            4 => Some(Num::Four),
            5 => Some(Num::Five),
            6 => Some(Num::Six),
            7 => Some(Num::Seven),
            8 => Some(Num::Eight),
            9 => Some(Num::Nine),
            _ => None
        }
    }
}
