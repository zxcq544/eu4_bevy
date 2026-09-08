use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use bevy_fluent::Localization;
use fluent_content::Content;

pub fn controls_button(
    top_block_with_video_audio_and_other_buttons: &mut RelatedSpawnerCommands<'_, ChildOf>,
    localization_res: &Res<Localization>,
    font: Handle<Font>,
) {
    let label = localization_res.content("controls").expect(&format!(
        "missing controls in localisation files {:?}",
        localization_res
    ));
    top_block_with_video_audio_and_other_buttons
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // bottom: Val::Px(3.0),
                ..default()
            },
            Outline {
                color: Color::srgb_from_array([0.7, 0.2, 0.5]),
                width: Val::Px(2.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    font: FontSource::Handle(font),
                    ..default()
                },
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
            ));
        });
}
