use crate::error::Result;
use crate::tile::Tile;

pub enum TileGroupLocation {
    Row(u8),
    Column(u8),
    House(u8, u8),
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
}
