use macroquad::prelude::*;
use core_sim::Grid;
use crate::view::camera::CameraState;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&self, grid: &Grid, camera: &CameraState, show_lines: bool) {
        clear_background(BLACK);

        // Optimisation : On ne dessine que les cellules visibles à l'écran
        // (View Culling simple)
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Taille visuelle d'une cellule (légèrement réduite pour faire un espace entre elles)
        let cell_size = camera.zoom;
        let gap = if camera.zoom > 5.0 { 1.0 } else { 0.0 }; // Pas d'espace si trop dézoomé
        let draw_size = cell_size - gap;

        let top_left = camera.world_to_screen(0.0, 0.0);
        let bottom_right = camera.world_to_screen(grid.width() as f32, grid.height() as f32);

        draw_rectangle(
            top_left.x,
            top_left.y,
            bottom_right.x - top_left.x,
            bottom_right.y - top_left.y,
            DARKGRAY
        );

        // On itère sur toutes les cellules
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if let Some(cell) = grid.get_cell(x as i32, y as i32) {
                    if cell.is_alive() {
                        let pos = camera.world_to_screen(x as f32, y as f32);

                        // Si la cellule est hors de l'écran, on skip (culling basique)
                        if pos.x + cell_size < 0.0 || pos.x > screen_w ||
                            pos.y + cell_size < 0.0 || pos.y > screen_h {
                            continue;
                        }

                        draw_rectangle(pos.x, pos.y, draw_size, draw_size, WHITE);
                    }
                }
            }
        }

        // Dessiner les bordures de la zone de la simulation
        if show_lines{
            draw_rectangle_lines(
                top_left.x,
                top_left.y,
                bottom_right.x - top_left.x,
                bottom_right.y - top_left.y,
                2.0,
                DARKBLUE
            );
        }
    }
}