use crate::domain::cell::{Cell, CellState};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::default(); width * height];
        Self { width, height, cells }
    }

    #[inline]
    fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            Some((y as usize) * self.width + (x as usize))
        } else {
            None
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        self.get_index(x, y).map(|idx| &self.cells[idx])
    }

    pub fn set_cell(&mut self, x: i32, y: i32, state: CellState) {
        if let Some(idx) = self.get_index(x, y) {
            self.cells[idx] = Cell::new(state);
        }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn par_iter_mut(&mut self) -> rayon::slice::IterMut<'_, Cell> {
        self.cells.par_iter_mut()
    }
}