use bevy::prelude::*;
use bevy::window::WindowPlugin;
use eu4_bevy::core::GamePlugin;

fn main() {
    // Check for Europa Unversalis 4 folder location being present in settings and on disk
    let eu4_settings = settings::get_eu4_settings();
    App::new()
        .insert_resource(eu4_settings)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Europa Universalis 4".into(),
                visible: false,
                ..default()
            }),
            ..default()
        }))
        // Pull in all game systems via one root plugin
        .add_plugins(GamePlugin)
        .run();
}
