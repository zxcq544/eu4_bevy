use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryWindow, WindowPlugin, WindowResolution};
use eu4_bevy::*;
use settings::Settings;

fn main() {
    // Check for Europa Unversalis 4 folder location being present in settings and on disk
    let eu4_settings = settings::get_eu4_settings();
    App::new()
        // .add_systems(PreStartup, setup_window_monitor)
        .insert_resource(eu4_settings)
        .add_systems(
            Startup,
            (
                print_monitors.before(setup_window_monitor_using_resource),
                setup_window_monitor_using_resource,
            ),
        )
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Europa Universalis 4".into(),
                ..default()
            }),
            ..default()
        }))
        // Pull in all game systems via one root plugin
        .add_plugins(GamePlugin)
        .run();
}

fn setup_window_monitor_using_resource(
    settings: Res<Settings>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let monitor_index = settings.monitor_index;
    info!("Monitor index from settings: {}", monitor_index);
    if let Ok(mut window) = window_query.single_mut() {
        // This explicitly moves and centers the window on Monitor 0 (or 1, etc.)
        window.resolution = WindowResolution::new(1920 / 2, 1080 / 2);
        window.position = WindowPosition::Centered(MonitorSelection::Index(monitor_index));
    }
}

fn print_monitors(q_monitors: Query<Entity, With<Monitor>>) {
    let count = q_monitors.iter().count();
    info!("Found {} monitors", count);
    for (i, entity) in q_monitors.iter().enumerate() {
        info!("Monitor {}: {:?}", i, entity);
    }
}
