use std::fmt;

use crate::digit::Digit;
use crate::tile::Tile::{Known, Possibilities};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Known(Digit),
    Possibilities([bool; 9]),
}

impl Tile {
    pub fn new_known(digit: u8) -> Tile {
        Tile::Known(Digit::new(digit))
    }

    pub fn new_blank() -> Tile {
        Tile::Possibilities([true, true, true, true, true, true, true, true, true])
    }

    pub fn remove_possibility(self, num: Digit) -> Tile {
        let t = match self {
            Possibilities(mut arr) => {
                arr[(num.to_int() - 1) as usize] = false;
                Possibilities(arr)
            }
            _ => self,
        };
        t.reconcile()
    }

    fn reconcile(self) -> Tile {
        match self {
            Possibilities(arr) => {
                let possibilities = arr.iter().filter(|p| **p).fold(0, |acc, _| acc + 1);
                match possibilities {
                    0 => unreachable!("Tile with no possibilities"),
                    1 => arr
                        .iter()
                        .enumerate()
                        .filter_map(|(i, possible)| match *possible {
                            true => Some(Tile::Known(Digit::new(i as u8 + 1))),
                            false => None,
                        })
                        .next()
                        .unwrap(),
                    _ => Tile::Possibilities(arr),
                }
            }
            Known(_) => self,
        }
    }

    #[allow(dead_code)]
    pub fn digit(&self) -> Option<Digit> {
        if let Tile::Known(n) = self {
            return Some(*n);
        }
        None
    }

    pub(crate) fn to_test_string(&self) -> String {
        match self {
            Known(n) => format!("{}", n),
            Possibilities(_) => ".".to_string(),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Known(n) => write!(f, "{}", n),
            Possibilities(_) => write!(f, " "),
        }
    }
}
