use macroquad::prelude::*;
use core_sim::{Simulator, CellState};
use crate::view::camera::CameraState;

pub fn handle_mouse(simulator: &mut Simulator, camera: &CameraState) {
    if is_mouse_button_down(MouseButton::Left) {
        // Récupérer la position souris
        let (mx, my) = mouse_position();

        // Conversion Raycasting : Écran -> Monde -> Grille
        let (grid_x, grid_y) = camera.screen_to_world(mx, my);

        // Modifier la cellule (Si on est dans les limites)
        simulator.current_grid_mut().set_cell(grid_x, grid_y, CellState::Alive);
    }
}