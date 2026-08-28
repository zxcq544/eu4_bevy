use bevy::window::WindowPlugin;
use bevy::{diagnostic::DiagnosticsPlugin, prelude::*};
use bevy_fluent::{FluentPlugin, Locale};
use eu4_bevy::core::game::GamePlugin;
use unic_langid::langid;

// TODO: take localisation from settings
fn main() {
    // Check for Europa Unversalis 4 folder location being present in settings and on disk
    let eu4_settings = settings::get_eu4_settings();
    App::new()
        .insert_resource(eu4_settings)
        .insert_resource(Locale::new(langid!("ru-RU")))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Europa Universalis 4".into(),
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .disable::<bevy::log::LogPlugin>()
                .disable::<DiagnosticsPlugin>(),
        )
        .add_plugins(FluentPlugin)
        // Pull in all game systems via one root plugin
        .add_plugins(GamePlugin)
        // .insert_resource(WinitSettings::desktop_app()) // MacOS only I guess. Slows down everything on Windows
        .run();
}
