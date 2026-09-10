use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use bevy_fluent::Localization;
use fluent_content::Content;
use crate::plugins::options::systems::options_top_buttons_sytems::options_top_buttons_system_united::{OptionsTopTabButton, OptionsTopTabButtonAction};

pub fn game_button(
    options_top_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    localization_res: &Res<Localization>,
    font: Handle<Font>,
) {
    let label = localization_res.content("game").expect(&format!(
        "missing game in localisation files {:?}",
        localization_res
    ));
    options_top_block
        .spawn((
            Button,
            OptionsTopTabButton,
            OptionsTopTabButtonAction::Game,
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
