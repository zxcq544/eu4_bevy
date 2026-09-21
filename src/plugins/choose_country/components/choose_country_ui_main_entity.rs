use bevy::prelude::*;
use bevy_fluent::Localization;

use crate::{
    plugins::{
        choose_country::components::{
            bottom_left_block_entity_for_choose_country::bottom_left_block_entity_for_choose_country,
            bottom_middle_block_entity_for_choose_country::{
                SharedUiMaterials, bottom_middle_block_entity_for_choose_country,
            },
        },
        game_main::resources::game_ui_resources::GameUiResources,
    },
    shared::fonts::fonts::FontHandles,
};

#[derive(Component, Clone, Default)]
pub struct ChooseCountryUiMainEntity;

impl ChooseCountryUiMainEntity {
    pub fn spawn_using_commands(
        mut commands: Commands,
        localization_res: &Res<Localization>,
        fonts: &Res<FontHandles>,
        game_ui_resources: &Res<GameUiResources>,
        ui_materials_res: Res<SharedUiMaterials>,
    ) {
        commands
            .spawn((
                ChooseCountryUiMainEntity,
                Node {
                    display: Display::None,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                //Visibility::Hidden, // Spawn as hidden because we don't despawn ingame ui elements
                BackgroundColor(Color::NONE),
            ))
            .with_children(|main_node| {
                // main block non absolute for children
                main_node
                    .spawn((
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        // Outline {
                        //     color: Color::srgb(0.9, 0.9, 0.1),
                        //     width: Val::Px(2.0),
                        //     ..default()
                        // },
                    ))
                    .with_children(|main_block_for_children| {
                        // top block
                        main_block_for_children
                            .spawn((
                                Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(25.0),
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                },
                                // Outline {
                                //     color: Color::srgb(0.9, 0.2, 0.1),
                                //     width: Val::Px(2.0),
                                //     ..default()
                                // },
                            ))
                            .with_children(|top_block| {
                                // top left block
                                top_block.spawn((
                                    Node {
                                        display: Display::Flex,
                                        width: Val::Percent(30.0),
                                        height: Val::Percent(100.0),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    Outline {
                                        color: Color::srgb(0.1, 0.9, 0.5),
                                        width: Val::Px(2.0),
                                        ..default()
                                    },
                                ));
                                // top middle block
                                top_block.spawn((
                                    Node {
                                        display: Display::Flex,
                                        width: Val::Percent(30.0),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    Outline {
                                        color: Color::srgb(0.1, 0.3, 0.9),
                                        width: Val::Px(2.0),
                                        ..default()
                                    },
                                ));
                                // top right block
                                top_block.spawn((
                                    Node {
                                        display: Display::Flex,
                                        width: Val::Percent(30.0),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    Outline {
                                        color: Color::srgb(0.9, 0.9, 0.1),
                                        width: Val::Px(2.0),
                                        ..default()
                                    },
                                ));
                            });
                        // bottom block
                        main_block_for_children
                            .spawn((
                                Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(25.0),
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                },
                                // Outline {
                                //     color: Color::srgb(0.9, 0.9, 0.1),
                                //     width: Val::Px(2.0),
                                //     ..default()
                                // },
                            ))
                            .with_children(|bottom_block| {
                                // bottom left block
                                bottom_block
                                    .spawn((
                                        Node {
                                            display: Display::Flex,
                                            flex_direction: FlexDirection::Column,
                                            width: Val::Percent(30.0),
                                            height: Val::Percent(100.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        Outline {
                                            color: Color::srgb(0.9, 0.1, 0.1),
                                            width: Val::Px(2.0),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|bottom_left_block| {
                                        bottom_left_block_entity_for_choose_country(
                                            bottom_left_block,
                                            &game_ui_resources,
                                            &localization_res,
                                            &fonts,
                                        );
                                    });
                                // bottom middle block
                                bottom_block
                                    .spawn((
                                        Node {
                                            display: Display::Flex,
                                            flex_direction: FlexDirection::Column,
                                            width: Val::Percent(40.0),
                                            height: Val::Percent(100.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        Outline {
                                            color: Color::srgb(0.1, 0.9, 0.1),
                                            width: Val::Px(2.0),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|bottom_middle_block| {
                                        bottom_middle_block_entity_for_choose_country(
                                            bottom_middle_block,
                                            &localization_res,
                                            &fonts,
                                            &game_ui_resources,
                                            ui_materials_res,
                                        );
                                    });
                                // bottom right block
                                bottom_block.spawn((
                                    Node {
                                        display: Display::Flex,
                                        width: Val::Percent(30.0),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    Outline {
                                        color: Color::srgb(0.1, 0.1, 0.9),
                                        width: Val::Px(2.0),
                                        ..default()
                                    },
                                ));
                            });
                    });
            });
    }
}
