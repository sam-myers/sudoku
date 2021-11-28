use crate::error::Result;
use crate::tile_group::TileGroup;

pub trait Strategy {
    fn apply(group: TileGroup) -> Result<TileGroup>;
}
