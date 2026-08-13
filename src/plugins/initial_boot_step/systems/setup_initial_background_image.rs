use crate::plugins::initial_boot_step::components::main_menu_background::MainMenuBackground;
use bevy::{image::ImageLoaderSettings, prelude::*, render::render_resource::TextureFormat};

// use crate::MainMenuBackground;

pub fn setup_initial_background_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d::default(), MainMenuBackground));
    let background_filename = "gfx/loadingscreens/load_0.dds";
    // TODO: try to convert to bsn! and check how to free resources and components and so on
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::hidden(),
                ..default()
            },
            MainMenuBackground,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    ..default()
                },
                ImageNode {
                    image: asset_server
                        .load_builder()
                        .with_settings(|settings: &mut ImageLoaderSettings| {
                            // EU4 uses BGRA instead of RGBA at least for loading screen
                            settings.is_srgb = true;
                            settings.texture_format = Some(TextureFormat::Bgra8UnormSrgb);
                        })
                        .load(background_filename),
                    ..default()
                },
            ));
        });
}
