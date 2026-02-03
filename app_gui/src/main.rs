mod view;

use macroquad::prelude::*;
use core_sim::{Simulator, ConwayRule, CellState};
use crate::view::camera::CameraState;
use crate::view::renderer::Renderer;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Cell Simulation".to_owned(),
        window_width: 1920,
        window_height: 1080,
        // --- FULL SCREEN ---
        fullscreen: true,
        // Important pour les écrans modernes
        high_dpi: true,
        // Permet de redimensionner la fenêtre à la souris
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // 1. Initialisation du Core
    let rule = Box::new(ConwayRule);
    let grid_size = 100;
    let margin_r_pixel = 50.0;
    let margin_t_pixel = margin_r_pixel;
    let mut simulator = Simulator::new(grid_size, grid_size, rule);

    // Initialisons un motif sympa (Glider) pour voir quelque chose
    {
        let grid = simulator.current_grid_mut();
        grid.set_cell(10, 10, CellState::Alive);
        grid.set_cell(11, 11, CellState::Alive);
        grid.set_cell(11, 12, CellState::Alive);
        grid.set_cell(10, 12, CellState::Alive);
        grid.set_cell(9, 12, CellState::Alive);
    }

    // 2. Initialisation de la Vue
    let mut camera = CameraState::new();

    let grid_pixel = (grid_size as f32) * camera.zoom;

    // On règle la caméra
    camera.offset = vec2((screen_width() - grid_pixel - margin_r_pixel ), margin_t_pixel);

    let renderer = Renderer::new();

    // 3. Boucle Principale
    loop {
        // --- UPDATE ---

        // Gestion Caméra (Zoom/Pan)
        camera.update();

        // Simulation
        if is_key_pressed(KeyCode::Space) {
            simulator.step();
        }

        // --- DRAW ---
        renderer.draw(simulator.current_grid(), &camera);

        // Afficher des infos de debug
        draw_text("Space: Step | Right Click: Pan | Wheel: Zoom", 10.0, 30.0, 20.0, YELLOW);
        draw_text(&format!("Zoom: {:.2}", camera.zoom), 10.0, 50.0, 20.0, YELLOW);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 70.0, 20.0, GREEN);
        draw_text(&format!("x: {}", camera.offset.x), 10.0, 90.0, 20.0, WHITE);
        draw_text(&format!("y: {}", camera.offset.y), 10.0, 110.0, 20.0, WHITE);

        next_frame().await
    }
}