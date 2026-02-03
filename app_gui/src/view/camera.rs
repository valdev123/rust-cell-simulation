use macroquad::prelude::*;

pub struct CameraState {
    pub zoom: f32,
    pub offset: Vec2, // Décalage (Pan) en pixels
}

impl CameraState {
    pub fn new() -> Self {
        Self {
            zoom: 20.0, // Par défaut, 1 cellule = 20 pixels
            offset: vec2(0.0, 0.0),
        }
    }

    /// Convertit une coordonnée grille (x, y) en coordonnée écran (pixels)
    pub fn world_to_screen(&self, grid_x: f32, grid_y: f32) -> Vec2 {
        vec2(
            grid_x * self.zoom + self.offset.x,
            grid_y * self.zoom + self.offset.y
        )
    }

    /// Convertit une coordonnée écran (souris) en coordonnée grille
    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> (i32, i32) {
        let x = ((screen_x - self.offset.x) / self.zoom).floor() as i32;
        let y = ((screen_y - self.offset.y) / self.zoom).floor() as i32;
        (x, y)
    }

    /// Gère les inputs (Molette souris + Clic droit pour bouger)
    pub fn update(&mut self) {
        // 1. Zoom avec la molette
        let (_, wheel_y) = mouse_wheel(); // On récupère le Y du tuple

        if wheel_y != 0.0 {
            let mouse_pos = mouse_position();
            let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);

            // On calcule où pointe la souris dans le monde AVANT le zoom
            let world_before = (mouse_vec - self.offset) / self.zoom;

            // Appliquer le zoom (clamp pour éviter d'aller trop loin)
            self.zoom *= if wheel_y > 0.0 { 1.1 } else { 0.9 };
            self.zoom = self.zoom.clamp(1.0, 100.0);

            // On recalcule l'offset pour que la souris reste au même endroit sous le curseur
            let new_offset = mouse_vec - world_before * self.zoom;
            self.offset = new_offset;
        }

        // 2. Panoramique (Drag avec clic droit ou molette)
        if is_mouse_button_down(MouseButton::Right) || is_mouse_button_down(MouseButton::Middle) {
            let delta = mouse_delta_position(); // Macroquad donne le delta normalisé (-1 à 1)

            // Il faut convertir le delta normalisé en pixels écran
            let delta_pixels = vec2(delta.x * screen_width(), delta.y * screen_height());

            self.offset += delta_pixels;
        }
    }
}