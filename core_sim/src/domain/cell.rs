use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CellState {
    Dead = 0,
    Alive = 1,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn new(state: CellState) -> Self {
        Self { state }
    }
    pub fn dead() -> Self {
        Self { state: CellState::Dead }
    }
    pub fn alive() -> Self {
        Self { state: CellState::Alive }
    }
    pub fn is_alive(&self) -> bool {
        self.state == CellState::Alive
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::dead()
    }
}