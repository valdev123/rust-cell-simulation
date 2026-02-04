use egui_macroquad::egui;
use macroquad::window::screen_width;
use core_sim::Simulator;
use crate::state::app_state::AppState;
use crate::view::camera::CameraState;

pub fn render_sidebar(ctx: &egui::Context, state: &mut AppState, sim: &Simulator, camera: &mut CameraState) {
    egui::SidePanel::left("controls_panel")
        .max_width(screen_width() / 5.0)
        .min_width(screen_width() / 5.0)
        .show(ctx, |ui| {
            ui.heading("Contrôles");
            ui.separator();

            // --- Section Playback ---
            ui.horizontal(|ui| {
                let icon = if state.is_paused { "▶" } else { "⏸" };
                let label = if state.is_paused { "Play" } else { "Pause" };

                if ui.button(format!("{} {}", icon, label)).clicked() {
                    state.is_paused = !state.is_paused;
                }

                // Bouton Step seulement si en pause
                if state.is_paused {
                    if ui.button("⏭ Step").clicked() {
                        state.is_step_clicked = true;
                    }
                    else {
                        state.is_step_clicked = false;
                    }
                }
            });

            ui.separator();

            // --- Section Vitesse ---
            ui.label(format!("Vitesse: {:.1} gen/s", state.simulation_speed));
            ui.add(egui::Slider::new(&mut state.simulation_speed, 1.0..=60.0).text("Speed"));

            ui.separator();
            ui.label(format!("Zoom: {:.1}x", camera.zoom));
            // On clamp entre 1.0 et 100.0
            ui.add(egui::Slider::new(&mut camera.zoom, 1.0..=100.0).text("Zoom"));

            ui.separator();

            // --- Section Infos ---
            ui.heading("📊 Statistiques");
            ui.label(format!("Génération: {}", sim.generation()));

            ui.separator();
            ui.checkbox(&mut state.show_grid_lines, "Afficher Grille");

            ui.separator();
            ui.label(format!("Mouse on panel: {}", state.is_mouse_captured_by_ui));

            ui.separator();
            ui.collapsing("Aide", |ui| {
                ui.label("Clic Droit: Déplacer la caméra");
                ui.label("Molette: Zoomer");
                ui.label("Espace: Play/Pause");
            });
        });
}