use crate::plugins::options::components::options_top_tabs_row_entity::{
    OptionsButton, OptionsButtonAction,
};
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use bevy_fluent::Localization;
use fluent_content::Content;

pub fn back_button(
    bottom_buttons_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    localization_res: &Res<Localization>,
    font: Handle<Font>,
    image: Handle<Image>,
    action_enum: OptionsButtonAction,
) {
    let label = localization_res.content("back").expect(&format!(
        "missing back in localisation files {:?}",
        localization_res
    ));
    bottom_buttons_block
        .spawn((
            Button,
            OptionsButton,
            action_enum,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(40.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // bottom: Val::Px(3.0),
                ..default()
            },
            Outline {
                color: Color::srgb_from_array([0.2, 0.7, 0.2]),
                width: Val::Px(2.0),
                ..default()
            },
            ImageNode {
                image: image,
                image_mode: NodeImageMode::Stretch,
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
