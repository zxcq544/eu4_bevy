use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowPosition, WindowResolution};
use settings::Settings;

pub fn setup_window_monitor(
    settings: Res<Settings>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let monitor_index = settings.monitor_index;
    let resolution_width = settings.resolution_width;
    let resolution_height = settings.resolution_height;
    info!("Monitor index from settings: {}", monitor_index);
    if let Ok(mut window) = window_query.single_mut() {
        // This explicitly moves and centers the window on Monitor 0 (or 1, etc.)
        window.resolution = WindowResolution::new(resolution_width, resolution_height);
        window.position = WindowPosition::Centered(MonitorSelection::Index(monitor_index));
        window.visible = true;
    }
}
