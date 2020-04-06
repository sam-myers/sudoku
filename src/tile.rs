use std::fmt;

use crate::num::Num;
use crate::tile::Tile::{Known, Possibilities};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Known(Num),
    Possibilities([bool; 9]),
}

impl Tile {
    pub fn new_blank() -> Tile {
        Tile::Possibilities([true, true, true, true, true, true, true, true, true])
    }

    pub fn remove_possibility(self, num: &Num) -> Tile {
        let t = match self {
            Possibilities(mut arr) => {
                arr[(num.to_int()-1) as usize] = false;
                Possibilities(arr)
            },
            _ => self,
        };
        t.reconcile()
    }

    fn reconcile(self) -> Tile {
        let t = match self {
            Possibilities([true, false, false, false, false, false, false, false, false]) => Tile::Known(Num::One),
            Possibilities([false, true, false, false, false, false, false, false, false]) => Tile::Known(Num::Two),
            Possibilities([false, false, true, false, false, false, false, false, false]) => Tile::Known(Num::Three),

            Possibilities([false, false, false, true, false, false, false, false, false]) => Tile::Known(Num::Four),
            Possibilities([false, false, false, false, true, false, false, false, false]) => Tile::Known(Num::Five),
            Possibilities([false, false, false, false, false, true, false, false, false]) => Tile::Known(Num::Six),

            Possibilities([false, false, false, false, false, false, true, false, false]) => Tile::Known(Num::Seven),
            Possibilities([false, false, false, false, false, false, false, true, false]) => Tile::Known(Num::Eight),
            Possibilities([false, false, false, false, false, false, false, false, true]) => Tile::Known(Num::Nine),

            Possibilities([false, false, false, false, false, false, false, false, false]) => { panic!("broken invariant: no possibilities left") },
            Possibilities(arr) => Tile::Possibilities(arr),
            Known(_) => self,
        };
        return t
    }

    pub fn num(&self) -> Option<Num> {
        if let Tile::Known(n) = self {
            return Some(n.clone())
        }
        None
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Known(n) => write!(f, "{}", n.to_int()),
            Possibilities(_) => write!(f, "{}", " "),
        }
    }
}
