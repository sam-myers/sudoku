use crate::digit::Digit;
use crate::error::SudokuError::InternalConsistencyError;
use crate::error::{Result, SudokuError};
use crate::tile::Tile;
use std::fmt;

pub enum TileGroupLocation {
    Row(u8),
    Column(u8),
    House(u8, u8),
}

impl fmt::Display for TileGroupLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TileGroupLocation::Column(i) => write!(f, "Column({})", i),
            TileGroupLocation::Row(i) => write!(f, "Row({})", i),
            TileGroupLocation::House(i, j) => write!(f, "House({}, {})", i, j),
        }
    }
}

pub struct TileGroup {
    pub location: TileGroupLocation,
    pub tiles: [Tile; 9], // This is easier to work with when building up
}

impl TileGroup {
    pub fn new(location: TileGroupLocation, tiles: [Tile; 9]) -> Result<Self> {
        let new = Self { tiles, location };
        new.validate()?;
        Ok(new)
    }

    fn validate(&self) -> Result<()> {
        self.validate_no_two_known()?;
        self.validate_all_digits_possible()?;
        Ok(())
    }

    fn validate_no_two_known(&self) -> Result<()> {
        let digits = self
            .tiles
            .iter()
            .filter_map(|t| (*t).digit())
            .collect::<Vec<Digit>>();

        for digit in 1..=9 {
            let digits_count = digits
                .iter()
                .filter(|d| (*d).to_int() == digit)
                .fold(0, |acc, _| acc + 1);

            if digits_count > 1 {
                return Err(SudokuError::InternalConsistencyError(
                    "Two or more digits in the same group",
                ));
            }
        }
        Ok(())
    }

    fn validate_all_digits_possible(&self) -> Result<()> {
        for digit in 1..=9 {
            let digit_known = self
                .tiles
                .iter()
                .filter_map(|t| (*t).digit())
                .any(|d| d.to_int() == digit);

            if digit_known {
                continue;
            }

            let tiles_possible = self
                .tiles
                .iter()
                .filter_map(|t| (*t).possibilities())
                .filter(|p| p[(digit - 1) as usize])
                .fold(0, |acc, _| acc + 1);

            if tiles_possible < 1 {
                return Err(InternalConsistencyError(
                    "All possibilities for digit removed",
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn to_test_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            self.tiles[0].to_test_string(),
            self.tiles[1].to_test_string(),
            self.tiles[2].to_test_string(),
            self.tiles[3].to_test_string(),
            self.tiles[4].to_test_string(),
            self.tiles[5].to_test_string(),
            self.tiles[6].to_test_string(),
            self.tiles[7].to_test_string(),
            self.tiles[8].to_test_string()
        )
    }
}

impl fmt::Display for TileGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.location, self.to_test_string(),)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::SudokuError;
    use crate::tile::Tile;
    use crate::tile_group::{TileGroup, TileGroupLocation};

    #[test]
    fn test_validate_all_unknown() {
        let group = TileGroup {
            location: TileGroupLocation::Column(0),
            tiles: [
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
            ],
        };
        assert!(matches!(group.validate(), Ok(()),))
    }

    #[test]
    fn test_validate_all_known() {
        let group = TileGroup {
            location: TileGroupLocation::Column(0),
            tiles: [
                Tile::new_known(1),
                Tile::new_known(2),
                Tile::new_known(3),
                Tile::new_known(4),
                Tile::new_known(5),
                Tile::new_known(6),
                Tile::new_known(7),
                Tile::new_known(8),
                Tile::new_known(9),
            ],
        };
        assert!(matches!(group.validate(), Ok(()),))
    }

    #[test]
    fn test_validate_no_two_known() {
        let group = TileGroup {
            location: TileGroupLocation::Column(0),
            tiles: [
                Tile::new_known(1),
                Tile::new_known(1),
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
                Tile::new_blank(),
            ],
        };
        assert!(matches!(
            group.validate(),
            Err(SudokuError::InternalConsistencyError(_)),
        ))
    }

    #[test]
    fn test_validate_possibilities() {
        let group = TileGroup {
            location: TileGroupLocation::Column(0),
            tiles: [
                Tile::Possibilities([true, false, false, false, false, false, false, false, false]),
                Tile::Possibilities([true, true, false, false, false, false, false, false, false]),
                Tile::Possibilities([true, true, true, false, false, false, false, false, false]),
                Tile::Possibilities([true, true, true, true, false, false, false, false, false]),
                Tile::Possibilities([true, true, true, true, true, false, false, false, false]),
                Tile::Possibilities([true, true, true, true, true, true, false, false, false]),
                Tile::Possibilities([true, true, true, true, true, true, true, false, false]),
                Tile::Possibilities([true, true, true, true, true, true, true, true, false]),
                Tile::Possibilities([true, true, true, true, true, true, true, true, true]),
            ],
        };
        assert!(matches!(group.validate(), Ok(())))
    }

    #[test]
    fn test_validate_no_possibilities() {
        let group = TileGroup {
            location: TileGroupLocation::Column(0),
            tiles: [
                Tile::Possibilities([true, true, true, true, true, true, true, true, false]),
                Tile::Possibilities([true, true, true, true, true, true, true, true, false]),
                Tile::Possibilities([true, true, true, true, true, true, true, true, false]),
                Tile::Possibilities([true, true, true, true, true, true, true, true, false]),
                Tile::Possibilities([true, true, true, true, true, true, true, true, false]),
                Tile::Possibilities([true, true, true, true, true, true, true, true, false]),
                Tile::Possibilities([true, true, true, true, true, true, true, true, false]),
                Tile::Possibilities([true, true, true, true, true, true, true, true, false]),
                Tile::Possibilities([true, true, true, true, true, true, true, true, false]),
            ],
        };
        assert!(matches!(
            group.validate(),
            Err(SudokuError::InternalConsistencyError(_))
        ))
    }

    #[test]
    fn test_validate_no_possibilities_some_known() {
        let group = TileGroup {
            location: TileGroupLocation::Column(0),
            tiles: [
                Tile::new_known(2),
                Tile::Possibilities([false, false, true, true, true, true, true, true, true]),
                Tile::Possibilities([false, false, true, true, true, true, true, true, true]),
                Tile::Possibilities([false, false, true, true, true, true, true, true, true]),
                Tile::Possibilities([false, false, true, true, true, true, true, true, true]),
                Tile::Possibilities([false, false, true, true, true, true, true, true, true]),
                Tile::Possibilities([false, false, true, true, true, true, true, true, true]),
                Tile::Possibilities([false, false, true, true, true, true, true, true, true]),
                Tile::Possibilities([false, false, true, true, true, true, true, true, true]),
            ],
        };
        assert!(matches!(
            group.validate(),
            Err(SudokuError::InternalConsistencyError(_))
        ))
    }
}
