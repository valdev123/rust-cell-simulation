use crate::domain::grid::Grid;
use crate::domain::cell::CellState;
use crate::rules::traits::SimulationRule;

pub struct ConwayRule;

impl SimulationRule for ConwayRule {
    fn name(&self) -> &'static str { "Conway's Game of Life" }

    fn apply(&self, x: i32, y: i32, grid: &Grid) -> CellState {
        let current = grid.get_cell(x, y).unwrap();
        let mut neighbors = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                if let Some(c) = grid.get_cell(x + dx, y + dy) {
                    if c.is_alive() { neighbors += 1; }
                }
            }
        }

        match (current.state, neighbors) {
            (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
            (CellState::Dead, 3) => CellState::Alive,
            _ => CellState::Dead,
        }
    }
}