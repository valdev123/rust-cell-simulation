use crate::domain::grid::Grid;
use crate::domain::cell::Cell;
use crate::rules::traits::SimulationRule;
use rayon::prelude::*;

pub struct Simulator {
    grid_a: Grid,
    grid_b: Grid,
    is_a_active: bool,
    rule: Box<dyn SimulationRule>,
    generation: u64,
}

impl Simulator {
    pub fn new(width: usize, height: usize, rule: Box<dyn SimulationRule>) -> Self {
        Self {
            grid_a: Grid::new(width, height),
            grid_b: Grid::new(width, height),
            is_a_active: true,
            rule,
            generation: 0,
        }
    }

    pub fn current_grid(&self) -> &Grid {
        if self.is_a_active { &self.grid_a } else { &self.grid_b }
    }

    pub fn current_grid_mut(&mut self) -> &mut Grid {
        if self.is_a_active { &mut self.grid_a } else { &mut self.grid_b }
    }

    pub fn step(&mut self) {
        let (read, write) = if self.is_a_active {
            (&self.grid_a, &mut self.grid_b)
        } else {
            (&self.grid_b, &mut self.grid_a)
        };

        let width = read.width();

        write.par_iter_mut().enumerate().for_each(|(i, cell)| {
            let x = (i % width) as i32;
            let y = (i / width) as i32;
            let new_state = self.rule.apply(x, y, read);
            *cell = Cell::new(new_state);
        });

        self.is_a_active = !self.is_a_active;
        self.generation += 1;
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }
}

// === TEST UNITAIRE (LE GLIDER) ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::conway::ConwayRule;
    use crate::domain::cell::CellState;

    #[test]
    fn test_glider_movement() {
        let rule = Box::new(ConwayRule);
        let mut sim = Simulator::new(10, 10, rule);
        let grid = sim.current_grid_mut();

        // Glider en (0,0)
        grid.set_cell(1, 0, CellState::Alive);
        grid.set_cell(2, 1, CellState::Alive);
        grid.set_cell(0, 2, CellState::Alive);
        grid.set_cell(1, 2, CellState::Alive);
        grid.set_cell(2, 2, CellState::Alive);

        // Avance de 4 générations
        for _ in 0..4 { sim.step(); }

        // Vérification du déplacement
        let grid = sim.current_grid();
        assert!(grid.get_cell(2, 1).unwrap().is_alive());
        assert!(grid.get_cell(3, 2).unwrap().is_alive());
        assert!(grid.get_cell(3, 3).unwrap().is_alive());
    }
}