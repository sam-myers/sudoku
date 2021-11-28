use crate::error::Result;
use crate::strategy::Strategy;
use crate::tile::Tile;
use crate::tile_group::TileGroup;

pub struct SweepTileStrategy;

impl Strategy for SweepTileStrategy {
    fn apply(mut group: TileGroup) -> Result<TileGroup> {
        for d in 0..9 {
            if let Tile::Known(digit) = group.tiles[d] {
                for i in 0..9 {
                    group.tiles[i] = group.tiles[i].remove_possibility(digit);
                }
            }
        }

        group.validate()?;
        Ok(group)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digit::Digit;
    use crate::helpers::new_tile_group;
    use crate::tile::Tile;

    #[test]
    fn test_single_tile() {
        let mut group = new_tile_group("1........");
        group = SweepTileStrategy::apply(group).unwrap();
        assert_eq!(
            group.tiles[1],
            Tile::Possibilities([false, true, true, true, true, true, true, true, true,])
        )
    }

    #[test]
    fn test_seven_vertical() {
        let mut group = new_tile_group("1234567..");
        group = SweepTileStrategy::apply(group).unwrap();
        assert_eq!(
            group.tiles[8],
            Tile::Possibilities([false, false, false, false, false, false, false, true, true,])
        )
    }

    #[test]
    fn test_eight_vertical() {
        let mut group = new_tile_group("12345678.");
        group = SweepTileStrategy::apply(group).unwrap();
        assert_eq!(group.tiles[8], Tile::Known(Digit::new(9)))
    }
}
