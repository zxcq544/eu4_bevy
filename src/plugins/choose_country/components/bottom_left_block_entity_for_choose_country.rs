use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use bevy_fluent::Localization;
use fluent_content::Content;

use crate::{
    plugins::game_main::resources::game_ui_resources::GameUiResources,
    shared::fonts::fonts::FontHandles,
};

// Markers
#[derive(Component, Clone)]
pub enum BottomLeftBlockButtonForChooseCountry {
    Options,
    Back,
}

// Marker actions
#[derive(Component, Clone)]
pub enum BottomLeftBlockButtonActionsForChooseCountry {
    Options,
    Back,
}

pub fn bottom_left_block_entity_for_choose_country(
    bottom_left_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    game_ui_resources: &Res<GameUiResources>,
    localization_res: &Res<Localization>,
    fonts: &Res<FontHandles>,
) {
    let back_label = localization_res.content("back").expect(&format!(
        "missing back in localisation files {:?}",
        localization_res
    ));
    let options_label = localization_res.content("options").expect(&format!(
        "missing options in localisation files {:?}",
        localization_res
    ));
    bottom_left_block
        .spawn((
            // Options button
            Button,
            BottomLeftBlockButtonForChooseCountry::Options,
            BottomLeftBlockButtonActionsForChooseCountry::Options,
            Node {
                height: Val::Percent(20.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode {
                image: game_ui_resources.button_wide_image.clone(),
                image_mode: NodeImageMode::Stretch,
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(options_label),
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
                TextFont {
                    font_size: FontSize::Px(14.0),
                    font: FontSource::Handle(fonts.button_font.clone()),
                    ..default()
                },
            ));
        });
    bottom_left_block
        .spawn((
            // Back button
            Button,
            BottomLeftBlockButtonForChooseCountry::Back,
            BottomLeftBlockButtonActionsForChooseCountry::Back,
            Node {
                height: Val::Percent(20.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode {
                image: game_ui_resources.button_wide_image.clone(),
                image_mode: NodeImageMode::Stretch,
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(back_label),
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
                TextFont {
                    font_size: FontSize::Px(14.0),
                    font: FontSource::Handle(fonts.button_font.clone()),
                    ..default()
                },
            ));
        });
}
