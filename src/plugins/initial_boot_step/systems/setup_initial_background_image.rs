use crate::plugins::initial_boot_step::{
    components::main_menu_background::MainMenuBackground,
    resources::initial_booting_background_screen::InitialBootingBackgroundScreen,
};
use bevy::prelude::*;

pub fn setup_initial_background_image(
    mut commands: Commands,
    background_image: Res<InitialBootingBackgroundScreen>,
) {
    info!("Setting initial background image");
    commands.spawn((Camera2d::default(), MainMenuBackground));
    // TODO: try to convert to bsn! and check how to free resources and components and so on
    commands
        .spawn((
            MainMenuBackground,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::hidden(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    ..default()
                },
                ImageNode {
                    image: background_image.image.clone(),
                    ..default()
                },
            ));
        });
    // commands.spawn_scene(ui());
}

// fn ui() -> impl Scene {
//     bsn! {
//         MainMenuBackground
//         Node{
//             width: Val::Percent(100.0),
//             height: Val::Percent(100.0),
//             position_type: PositionType::Absolute,
//             justify_content: JustifyContent::Center,
//             align_items: AlignItems::Center,
//             overflow: Overflow::hidden()
//         }
//         Children[
//             Node{
//                 width: Val::Percent(100.0),
//             }
//             ImageNode{
//                 image: "gfx/loadingscreens/load_0.dds"
//             }
//         ]
//     }
// }
