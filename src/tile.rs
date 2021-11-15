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
            Possibilities([true, false, false, false, false, false, false, false, false]) => {
                Tile::Known(Digit::new(1))
            }
            Possibilities([false, true, false, false, false, false, false, false, false]) => {
                Tile::Known(Digit::new(2))
            }
            Possibilities([false, false, true, false, false, false, false, false, false]) => {
                Tile::Known(Digit::new(3))
            }

            Possibilities([false, false, false, true, false, false, false, false, false]) => {
                Tile::Known(Digit::new(4))
            }
            Possibilities([false, false, false, false, true, false, false, false, false]) => {
                Tile::Known(Digit::new(5))
            }
            Possibilities([false, false, false, false, false, true, false, false, false]) => {
                Tile::Known(Digit::new(6))
            }

            Possibilities([false, false, false, false, false, false, true, false, false]) => {
                Tile::Known(Digit::new(7))
            }
            Possibilities([false, false, false, false, false, false, false, true, false]) => {
                Tile::Known(Digit::new(8))
            }
            Possibilities([false, false, false, false, false, false, false, false, true]) => {
                Tile::Known(Digit::new(9))
            }

            Possibilities([false, false, false, false, false, false, false, false, false]) => {
                unreachable!()
            }
            Possibilities(arr) => Tile::Possibilities(arr),
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
