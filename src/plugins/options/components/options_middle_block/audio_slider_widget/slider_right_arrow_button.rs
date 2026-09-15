use crate::{
    plugins::game_main::resources::game_ui_resources::GameUiResources,
    shared::audio_channel::audio_channel::AudioChannel,
};
use bevy::prelude::*;
#[derive(Component, Default)]
pub struct OptionsUIAudioSliderButtonRight;

pub fn slider_right_arrow_button(
    ui_resources: &Res<GameUiResources>,
    audio_channel: AudioChannel,
) -> impl Bundle {
    (
        Button,
        OptionsUIAudioSliderButtonRight,
        audio_channel,
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
        //     color: Color::srgb_from_array([0.1, 0.9, 0.9]),
        //     width: Val::Px(2.0),
        //     ..default()
        // },
        ImageNode {
            image: ui_resources.ui_options_slider_right_arrow_image.clone(),
            image_mode: NodeImageMode::Stretch,
            ..default()
        },
    )
}
