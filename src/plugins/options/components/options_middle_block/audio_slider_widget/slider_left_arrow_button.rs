use crate::plugins::game_main::resources::game_ui_resources::GameUiResources;
use bevy::prelude::*;
#[derive(Component, Default)]
pub struct OptionsUIAudioSliderButtonLeft;

pub fn slider_left_arrow_button(ui_resources: &Res<GameUiResources>) -> impl Bundle {
    (
        Button,
        OptionsUIAudioSliderButtonLeft,
        Node {
            display: Display::Grid,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,           
            ..default()
        },
        ZIndex(6),
        // Outline {
        //     color: Color::srgb_from_array([0.9, 0.1, 0.9]),
        //     width: Val::Px(2.0),
        //     ..default()
        // },
        ImageNode {
            image: ui_resources.ui_options_slider_left_arrow_image.clone(),
            image_mode: NodeImageMode::Stretch,
            ..default()
        },
    )
}
