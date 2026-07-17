use bevy::prelude::*;
use bevy::window::{WindowPlugin, WindowResolution};
mod core;
use crate::core::game::GamePlugin;

fn main() {
    // Check for Europa Unversalis 4 folder location being present in settings and on disk
    let eu4_settings = crate::core::settings::get_eu4_settings();
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Europa Universalis 4".into(),
                        resolution: WindowResolution::new(800, 450),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // pixel-art friendly
        )
        // Pull in all game systems via one root plugin
        .add_plugins(GamePlugin)
        .run();
}
