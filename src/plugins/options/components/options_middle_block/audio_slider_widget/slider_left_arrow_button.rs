use crate::plugins::game_main::resources::game_ui_resources::GameUiResources;
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

pub fn slider_left_arrow_button(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    ui_resources: &Res<GameUiResources>,
) {
    parent.spawn((
        Button,
        Node {
            display: Display::Grid,
            // flex_direction: FlexDirection::Column,
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
        // observe(
        //     move |change: On<Pointer<Click>>, mut settings: ResMut<Settings>| {
        //         let volume = audio_channel.get(&settings);
        //         if volume > 0.0 {
        //             audio_channel.set(&mut settings, volume - 0.1);
        //         } else {
        //             // volume = 0;
        //             audio_channel.set(&mut settings, 0.0);
        //         }
        //     },
        // ),
    ));
}
