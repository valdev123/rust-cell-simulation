use macroquad::prelude::*;
use core_sim::{Simulator, CellState};
use crate::view::camera::CameraState;

pub fn handle_mouse(simulator: &mut Simulator, camera: &CameraState) {
    if is_mouse_button_down(MouseButton::Left) {
        // Récupérer la position souris
        let (mx, my) = mouse_position();
        // Conversion Raycasting : Écran -> Monde -> Grille
        let (grid_x, grid_y) = camera.screen_to_world(mx, my);

        let state_to_draw = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            CellState::Dead
        } else {
            CellState::Alive
        };

        // Modifier la cellule
        simulator.current_grid_mut().set_cell(grid_x, grid_y, state_to_draw);
    }
}