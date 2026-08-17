use crate::plugins::initial_boot_step::{
    components::main_menu_background::MainMenuBackground,
    resources::initial_booting_background_screen::InitialBootingBackgroundScreen,
};
use bevy::prelude::*;

pub fn setup_initial_background_image(
    mut commands: Commands,
    background_image_res: Res<InitialBootingBackgroundScreen>,
) {
    info!("Setting initial background image");
    commands.spawn((Camera2d::default(), MainMenuBackground));
    // TODO: check how to free resources and components and so on
    commands.spawn_scene(ui(background_image_res));
}

fn ui(background_image_res: Res<InitialBootingBackgroundScreen>) -> impl Scene {
    let background_image = background_image_res.image.clone();
    bsn! {
        MainMenuBackground
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            overflow: Overflow::hidden()
        }
        Children[
            Node{
                width: Val::Percent(100.0),
                height: Val::Auto,
            }
            ImageNode{
                image: background_image,
                image_mode: NodeImageMode::Auto,
            }
        ]
    }
}
