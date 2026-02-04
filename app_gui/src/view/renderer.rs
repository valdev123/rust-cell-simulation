use macroquad::prelude::*;
use core_sim::Grid;
use crate::view::camera::CameraState;
use rayon::prelude::*; // INDISPENSABLE pour la vitesse

pub struct Renderer {
    image: Image,
    texture: Texture2D,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let image = Image::gen_image_color(width as u16, height as u16, BLANK);
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest); // Pixel Art

        Self { image, texture }
    }

    pub fn draw(&mut self, grid: &Grid, camera: &CameraState, show_lines: bool) {
        // CALCUL DE LA GÉOMÉTRIE
        let dest_pos = camera.world_to_screen(0.0, 0.0);
        let dest_size_w = grid.width() as f32 * camera.zoom;
        let dest_size_h = grid.height() as f32 * camera.zoom;

        // FOND GLOBAL
        clear_background(BLACK);

        // FOND DE LA GRILLE
        draw_rectangle(dest_pos.x, dest_pos.y, dest_size_w, dest_size_h, DARKGRAY);

        // MISE À JOUR DES PIXELS (CPU -> Mémoire brute en parallèle)
        let cells = grid.raw_cells();
        let pixels = &mut self.image.bytes;

        cells.par_iter()
            .zip(pixels.par_chunks_exact_mut(4)) // 4 octets par pixel (R, G, B, A)
            .for_each(|(cell, pixel)| {
                if cell.is_alive() {
                    // Si vivant : BLANC
                    pixel[0] = 255;
                    pixel[1] = 255;
                    pixel[2] = 255;
                    pixel[3] = 255;
                } else {
                    // Si mort : TRANSPARENT
                    pixel[3] = 0;
                }
            });

        // UPLOAD VERS GPU
        self.texture.update(&self.image);

        // DESSIN DE LA TEXTURE (GPU)
        draw_texture_ex(
            &self.texture,
            dest_pos.x,
            dest_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_size_w, dest_size_h)),
                ..Default::default()
            },
        );

        // LIGNES DE LA GRILLE
        if show_lines && camera.zoom > 5.01 {
            self.draw_grid_lines(grid, camera, dest_pos, dest_size_w, dest_size_h);
        } else {
            draw_rectangle_lines(dest_pos.x, dest_pos.y, dest_size_w, dest_size_h, 2.0, BLUE);
        }
    }

    fn draw_grid_lines(&self, grid: &Grid, camera: &CameraState, start_pos: Vec2, total_w: f32, total_h: f32) {
        let color = Color::new(0.5, 0.5, 0.5, 0.2); // Gris transparent

        // Lignes verticales
        for x in 0..=grid.width() {
            let x_pos = start_pos.x + (x as f32 * camera.zoom);
            // Culling simple : on ne dessine pas si hors écran
            if x_pos >= 0.0 && x_pos <= screen_width() {
                draw_line(x_pos, start_pos.y, x_pos, start_pos.y + total_h, 1.0, color);
            }
        }

        // Lignes horizontales
        for y in 0..=grid.height() {
            let y_pos = start_pos.y + (y as f32 * camera.zoom);
            // Culling simple
            if y_pos >= 0.0 && y_pos <= screen_height() {
                draw_line(start_pos.x, y_pos, start_pos.x + total_w, y_pos, 1.0, color);
            }
        }
    }
}