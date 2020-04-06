use std::fmt;

use crate::tile::Tile;
use crate::num::Num;

pub struct Board {
    pub grid: [[Tile; 9]; 9],
    pub sweeps: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
            ],
            sweeps: 0,
        }
    }

    pub fn given(mut self, x: usize, y: usize, num: Num) -> Self {
        self.grid[x][y] = Tile::Known(num);
        self
    }

    pub fn is_done(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if let Tile::Possibilities(_) = self.grid[i][j] {
                    return false;
                }
            }
        }
        true
    }

    pub fn solve(&mut self) {
        while !self.is_done() && self.sweeps < 1000 {
            self.solve_sweep()
        }
    }

    pub fn solve_sweep(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                self.sweep_tile(i, j);
            }
        }
        self.sweeps += 1;
    }

    fn sweep_tile(&mut self, x: usize, y: usize) {
        if let Some(n) = &self.grid[x][y].num() {

            for i in 0..9 {
                // Vertical
                self.grid[i][y] = self.grid[i][y].remove_possibility(n);

                // Horizontal
                self.grid[x][i] = self.grid[x][i].remove_possibility(n);
            }

            let grid_x: usize = x / 3;
            let grid_y: usize = y / 3;

            for i in 0..3 {
                for j in 0..3 {
                    // Grid
                    self.grid[grid_x+i][grid_y+j] = self.grid[grid_x+i][grid_y+j].remove_possibility(n);
                }
            }
        }
    }

    fn new_row() -> [Tile; 9] {
        [
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
        ]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "╔══════════╦══════════╦══════════╗
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
╠══════════╬══════════╬══════════╣
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
╠══════════╬══════════╬══════════╣
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
╚══════════╩══════════╩══════════╝
",             &self.grid[0][8], &self.grid[1][8], &self.grid[2][8], &self.grid[3][8], &self.grid[4][8], &self.grid[5][8], &self.grid[6][8], &self.grid[7][8], &self.grid[8][8],
               &self.grid[0][7], &self.grid[1][7], &self.grid[2][7], &self.grid[3][7], &self.grid[4][7], &self.grid[5][7], &self.grid[6][7], &self.grid[7][7], &self.grid[8][7],
               &self.grid[0][6], &self.grid[1][6], &self.grid[2][6], &self.grid[3][6], &self.grid[4][6], &self.grid[5][6], &self.grid[6][6], &self.grid[7][6], &self.grid[8][6],

               &self.grid[0][5], &self.grid[1][5], &self.grid[2][5], &self.grid[3][5], &self.grid[4][5], &self.grid[5][5], &self.grid[6][5], &self.grid[7][5], &self.grid[8][5],
               &self.grid[0][4], &self.grid[1][4], &self.grid[2][4], &self.grid[3][4], &self.grid[4][4], &self.grid[5][4], &self.grid[6][4], &self.grid[7][4], &self.grid[8][4],
               &self.grid[0][3], &self.grid[1][3], &self.grid[2][3], &self.grid[3][3], &self.grid[4][3], &self.grid[5][3], &self.grid[6][3], &self.grid[7][3], &self.grid[8][3],

               &self.grid[0][2], &self.grid[1][2], &self.grid[2][2], &self.grid[3][2], &self.grid[4][2], &self.grid[5][2], &self.grid[6][2], &self.grid[7][2], &self.grid[8][2],
               &self.grid[0][1], &self.grid[1][1], &self.grid[2][1], &self.grid[3][1], &self.grid[4][1], &self.grid[5][1], &self.grid[6][1], &self.grid[7][1], &self.grid[8][1],
               &self.grid[0][0], &self.grid[1][0], &self.grid[2][0], &self.grid[3][0], &self.grid[4][0], &self.grid[5][0], &self.grid[6][0], &self.grid[7][0], &self.grid[8][0],
        )
    }
}
