pub struct AppState {
    pub is_paused: bool,
    pub simulation_speed: f32, // En étapes par seconde (Target FPS)
    pub show_grid_lines: bool,
    pub is_mouse_captured_by_ui: bool,
    pub is_step_clicked: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            is_paused: true,       // On démarre en pause
            simulation_speed: 10.0,
            show_grid_lines: true,
            is_mouse_captured_by_ui: false,
            is_step_clicked: false,
        }
    }
}