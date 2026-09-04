use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::window::WindowPlugin;
use bevy::{dev_tools::fps_overlay::FrameTimeGraphConfig, prelude::*};
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
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Europa Universalis 4".into(),
                    visible: false,
                    ..default()
                }),
                ..default()
            }), // .set(RenderPlugin {
                //     render_creation: RenderCreation::Automatic(Box::new(WgpuSettings {
                //         backends: Some(Backends::VULKAN),
                //         ..default()
                //     })),
                //     ..default()
                // })
                // .disable::<bevy::log::LogPlugin>()
                // .disable::<DiagnosticsPlugin>(),
        )
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    // Here we define size of our overlay
                    font_size: FontSize::Px(42.0),
                    // If we want, we can use a custom font
                    font: default(),
                    // We could also disable font smoothing,
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                // We can also change color of the overlay
                text_color: Color::WHITE,
                // We can also set the refresh interval for the FPS counter
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: true,
                    // The minimum acceptable fps
                    min_fps: 30.0,
                    // The target fps
                    target_fps: 144.0,
                },
            },
        })
        .add_plugins(FluentPlugin)
        // Pull in all game systems via one root plugin
        .add_plugins(GamePlugin)
        // .insert_resource(WinitSettings::desktop_app()) // MacOS only I guess. Slows down everything on Windows
        .run();
}
