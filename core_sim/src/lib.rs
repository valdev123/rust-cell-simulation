pub mod domain;
pub mod rules;
pub mod engine;

pub use domain::grid::Grid;
pub use domain::cell::{Cell, CellState};
pub use engine::simulator::Simulator;
pub use rules::conway::ConwayRule;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
