use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

#[derive(Component, Clone, Default)]
pub struct OptionsUiAudioTab;

pub fn audio_tab(
    options_middle_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    // localization_res: &Res<Localization>,
    // fonts: &Res<FontHandles>,
) {
    options_middle_block
        .spawn((
            OptionsUiAudioTab,
            Node {
                display: Display::None,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                // bottom: Val::Px(3.0),
                ..default()
            },
            // Outline {
            //     color: Color::srgb_from_array([0.9, 0.9, 0.1]),
            //     width: Val::Px(2.0),
            //     ..default()
            // },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    // bottom: Val::Px(3.0),
                    ..default()
                },
                Outline {
                    color: Color::srgb_from_array([0.9, 0.9, 0.1]),
                    width: Val::Px(2.0),
                    ..default()
                },
                // Text::new(localization_res.content("video").expect(&format!(
                //     "missing video in localisation files {:?}",
                //     localization_res
                // ))),
                Text::new("Audio tab content left"),
            ));
            parent.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // bottom: Val::Px(3.0),
                    ..default()
                },
                Outline {
                    color: Color::srgb_from_array([0.1, 0.1, 0.9]),
                    width: Val::Px(2.0),
                    ..default()
                },
                Text::new("Audio tab content right"),
            ));
        });
}
