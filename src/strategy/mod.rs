#[allow(clippy::module_inception)]
mod strategy;
mod strategy_sweep_tile;
mod solve;

pub use strategy::Strategy;
pub use strategy_sweep_tile::SweepTileStrategy;
pub use solve::solve;
