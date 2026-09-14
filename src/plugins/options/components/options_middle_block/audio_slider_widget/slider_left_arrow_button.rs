use crate::plugins::game_main::resources::game_ui_resources::GameUiResources;
use bevy::prelude::*;

pub fn slider_left_arrow_button(ui_resources: &Res<GameUiResources>) -> impl Bundle {
    (
        Button,
        Node {
            display: Display::Grid,
            // flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            // left: Val::Percent(50.0),
            // padding: UiRect::left(Val::Percent(12.5))
            //     .with_right(Val::Percent(12.5)),
            // grid_template_columns: vec![
            //     GridTrack::percent(12.5),
            //     GridTrack::percent(75.0),
            //     GridTrack::percent(12.5),
            // ],
            // grid_template_rows: vec![GridTrack::percent(12.5), GridTrack::percent(75.0)],
            // bottom: Val::Px(3.0),
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
    )
}
