use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraConfig {
    pub pan_speed: f32,
    pub zoom_speed: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub min_pitch: f32, // Pitch in radians when fully zoomed out (looking straight down: -PI/2)
    pub max_pitch: f32, // Pitch in radians when fully zoomed in (angled view: e.g., -PI/4)
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            pan_speed: 1.5,
            zoom_speed: 150.0,
            min_height: 200.0,
            max_height: 2000.0,
            min_pitch: -std::f32::consts::FRAC_PI_2, // -90 degrees (straight down)
            max_pitch: -std::f32::consts::FRAC_PI_4, // -45 degrees (looking forward-down)
        }
    }
}
