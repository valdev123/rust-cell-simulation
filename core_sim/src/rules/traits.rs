use crate::domain::grid::Grid;
use crate::domain::cell::CellState;

pub trait SimulationRule: Send + Sync {
    fn apply(&self, x: i32, y: i32, grid: &Grid) -> CellState;
    fn name(&self) -> &'static str;
}