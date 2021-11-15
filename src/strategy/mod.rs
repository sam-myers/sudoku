#[allow(clippy::module_inception)]
mod strategy;
mod strategy_sweep_tile;

pub use strategy::Strategy;
pub use strategy_sweep_tile::SweepTileStrategy;
