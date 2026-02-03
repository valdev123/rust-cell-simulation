pub struct AppState {
    pub is_paused: bool,
    pub simulation_speed: f32, // En étapes par seconde (Target FPS)
    pub show_grid_lines: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            is_paused: true,       // On démarre en pause
            simulation_speed: 10.0,
            show_grid_lines: true,
        }
    }
}