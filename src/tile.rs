use std::fmt;

use crate::digit::Digit;
use crate::tile::Tile::{Known, Possibilities};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Known(Digit),
    Possibilities([bool; 9]),
}

impl Tile {
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
                let only_one = arr.iter().filter(|p| **p).nth(1).is_none();
                match only_one {
                    true => arr
                        .iter()
                        .enumerate()
                        .filter_map(|(i, possible)| match *possible {
                            true => Some(Tile::Known(Digit::new(i as u8 + 1))),
                            false => None,
                        })
                        .next()
                        .unwrap(),
                    false => Tile::Possibilities(arr),
                }
            }
            Known(_) => self,
        }
    }

    #[allow(dead_code)]
    pub fn num(&self) -> Option<Digit> {
        if let Tile::Known(n) = self {
            return Some(*n);
        }
        None
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
