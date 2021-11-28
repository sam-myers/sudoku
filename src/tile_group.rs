use crate::error::Result;
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
        Self { tiles, location }.validate()
    }

    fn validate(self) -> Result<Self> {
        Ok(self)
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
