use std::fmt;

use crate::num::Num;
use crate::tile::Tile::{Known, Possibilities};

pub enum Tile {
    Known(Num),
    Possibilities([bool; 9]),
}

impl Tile {
    pub fn new_blank() -> Tile {
        Tile::Possibilities([true, true, true, true, true, true, true, true, true])
    }

    pub fn new_known(num: Num) -> Tile {
        Tile::Known(num)
    }

    pub fn remove_possibility(mut self, num: Num) -> Self {
        match self {
            Possibilities(mut arr) => {
                arr[(num.to_int()-1) as usize] = false;
                self.reconcile()
            },
            _ => self,
        }
    }

    pub fn reconcile(&self) -> Tile {
        match self {
            Possibilities([true, false, false, false, false, false, false, false, false]) => Tile::Known(Num::One),
            Possibilities([false, true, false, false, false, false, false, false, false]) => Tile::Known(Num::Two),
            Possibilities([false, false, true, false, false, false, false, false, false]) => Tile::Known(Num::Three),

            Possibilities([false, false, false, true, false, false, false, false, false]) => Tile::Known(Num::Four),
            Possibilities([false, false, false, false, true, false, false, false, false]) => Tile::Known(Num::Five),
            Possibilities([false, false, false, false, false, true, false, false, false]) => Tile::Known(Num::Six),

            Possibilities([false, false, false, false, false, false, true, false, false]) => Tile::Known(Num::Seven),
            Possibilities([false, false, false, false, false, false, false, true, false]) => Tile::Known(Num::Eight),
            Possibilities([false, false, false, false, false, false, false, false, true]) => Tile::Known(Num::Nine),

//            Possibilities([false, false, false, false, false, false, false, false, false]) => { panic!("broken invariant: no possibilities left"); self.clone() },
            Possibilities(arr) => Tile::Possibilities(*arr),
            Known(num) => Tile::Known(Num::from_int(num.to_int()).unwrap()),
        }
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
