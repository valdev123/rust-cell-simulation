mod view;
mod state;
mod ui;
mod input;

use macroquad::prelude::*;
use egui_macroquad::macroquad; // Nécessaire pour le pont
use core_sim::{Simulator, ConwayRule, CellState};
use crate::input::mouse::handle_mouse;
use crate::state::app_state;
use crate::view::camera::CameraState;
use crate::view::renderer::Renderer;
use crate::state::app_state::AppState;
use crate::ui::panels;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Cell Simulation".to_owned(),
        window_width: 1920,
        window_height: 1080,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // --- SETUP ---
    let rule = Box::new(ConwayRule);
    let grid_size = 100;
    let mut simulator = Simulator::new(grid_size, grid_size, rule);

    // Initialisation Glider
    {
        let grid = simulator.current_grid_mut();
        grid.set_cell(1, 0, CellState::Alive);
        grid.set_cell(2, 1, CellState::Alive);
        grid.set_cell(0, 2, CellState::Alive);
        grid.set_cell(1, 2, CellState::Alive);
        grid.set_cell(2, 2, CellState::Alive);
    }

    let mut camera = CameraState::new();
    let renderer = Renderer::new();
    let mut app_state = AppState::new();

    // Centrage Caméra
    camera.zoom = 9.0;
    let grid_pixel = (grid_size as f32) * camera.zoom;
    let margin_l_pixel = 30.0;
    let margin_t_pixel = 30.0;
    camera.offset = vec2(
        screen_width() / 5.0 + margin_l_pixel,
        margin_t_pixel,
    );

    // Accumulateur de temps pour la vitesse variable
    let mut time_accumulator = 0.0;

    // --- MAIN LOOP ---
    loop {
        // ============================================================
        // 1. INPUT PHASE (UI & Mouse)
        // ============================================================

        // ou si la souris est capturée.
        egui_macroquad::ui(|ctx| {
            panels::render_sidebar(ctx, &mut app_state, &simulator, &mut camera);
            app_state.is_mouse_captured_by_ui = ctx.wants_pointer_input() || ctx.is_pointer_over_area();
        });

        // Manage Input Game (Camera & Draw)
        if !app_state.is_mouse_captured_by_ui {
            camera.update();
            handle_mouse(&mut simulator, &camera);
        }

        // ============================================================
        // 2. UPDATE PHASE (Logical & Time)
        // ============================================================

        let dt = get_frame_time();
        time_accumulator += dt;

        let step_duration = 1.0 / app_state.simulation_speed;
        let mut should_step = false;

        if !app_state.is_paused {
            if time_accumulator >= step_duration {
                should_step = true;
                time_accumulator = 0.0;
            }
        }
        else if is_key_pressed(KeyCode::Space) || app_state.is_step_clicked {
            should_step = true;
        }

        if should_step {
            simulator.step();
        }

        // ============================================================
        // 3. RENDER PHASE (Dessin)
        // ============================================================

        renderer.draw(simulator.current_grid(), &camera, app_state.show_grid_lines);

        egui_macroquad::draw();

        next_frame().await
    }
}