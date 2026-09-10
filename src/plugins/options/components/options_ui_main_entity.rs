use crate::plugins::options::{
    components::{
        options_bottom_buttons::{apply_button::apply_button, back_button::back_button},
        options_middle_block::{audio_tab::audio_tab, video_tab::video_tab},
        options_top_buttons::{
            audio_button::audio_button, controls_button::controls_button, game_button::game_button,
            video_button::video_button,
        },
    },
    resources::options_images::OptionsImages,
};
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;
use settings::Settings;

#[derive(Component, Clone, Default, Reflect)]
pub struct OptionsButton;

#[derive(Component, Clone, Default)]
pub enum OptionsButtonAction {
    #[default]
    NoAction,
    Apply,
    Back,
}

#[derive(Component, Clone, Default)]
pub struct OptionsUiMainEntity;

impl OptionsUiMainEntity {
    pub fn spawn_using_commands(
        settings: ResMut<Settings>,
        mut commands: Commands,
        localization_res: &Res<Localization>,
        fonts: &Res<FontHandles>,
        options_images: &Res<OptionsImages>,
    ) {
        commands
            .spawn((
                OptionsUiMainEntity,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Visibility::Hidden, // Spawn as hidden because we don't despawn ingame ui elements
                ZIndex(2),
                BackgroundColor(Color::NONE),
            ))
            .with_children(|main_block| {
                main_block
                    .spawn((
                        // Options block with all controls
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(70.0),
                            height: Val::Vh(90.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            // bottom: Val::Px(3.0),
                            ..default()
                        },
                        Outline {
                            color: Color::srgb_from_array([0.4, 0.7, 0.5]),
                            width: Val::Px(2.0),
                            ..default()
                        },
                        ImageNode {
                            image: options_images.settings_bg_image.clone(),
                            image_mode: NodeImageMode::Stretch,
                            ..default()
                        },
                    ))
                    .with_children(|options_block| {
                        options_block
                            .spawn((
                                // Main smaller block with all controls
                                Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    width: Val::Percent(70.0),
                                    height: Val::Percent(55.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    top: Val::Percent(5.0),
                                    // bottom: Val::Px(3.0),
                                    ..default()
                                },
                                // Outline {
                                //     color: Color::srgb_from_array([0.7, 0.2, 0.5]),
                                //     width: Val::Px(2.0),
                                //     ..default()
                                // },
                            ))
                            .with_children(|main_smaller_block_with_all_controls| {
                                // Main settings block with all controls
                                main_smaller_block_with_all_controls
                                    .spawn((
                                        Node {
                                            display: Display::Flex,
                                            flex_direction: FlexDirection::Column,
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            justify_content: JustifyContent::FlexStart,
                                            align_items: AlignItems::Center,
                                            // bottom: Val::Px(3.0),
                                            ..default()
                                        },
                                        Outline {
                                            color: Color::srgb_from_array([0.4, 0.9, 0.5]),
                                            width: Val::Px(2.0),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|main_settings_block_with_all_controls| {
                                        // Top block with buttons Video, Audio, Game, Controls, etc.
                                        main_settings_block_with_all_controls
                                            .spawn((
                                                Node {
                                                    display: Display::Flex,
                                                    flex_direction: FlexDirection::Row,
                                                    width: Val::Percent(100.0),
                                                    height: Val::Percent(10.0),
                                                    justify_content: JustifyContent::SpaceBetween,
                                                    align_items: AlignItems::Center,
                                                    // bottom: Val::Px(3.0),
                                                    ..default()
                                                },
                                                Outline {
                                                    color: Color::srgb_from_array([0.1, 0.1, 0.9]),
                                                    width: Val::Px(2.0),
                                                    ..default()
                                                },
                                            ))
                                            .with_children(
                                                |top_block_with_video_audio_and_other_buttons| {
                                                    // Video button
                                                    video_button(
                                                        top_block_with_video_audio_and_other_buttons,
                                                        &localization_res,
                                                        fonts.button_font.clone(),
                                                    );
                                                    // Audio button
                                                    audio_button(
                                                        top_block_with_video_audio_and_other_buttons,
                                                        &localization_res,
                                                        fonts.button_font.clone(),
                                                    );
                                                    // Game button
                                                    game_button(
                                                        top_block_with_video_audio_and_other_buttons,
                                                        &localization_res,
                                                        fonts.button_font.clone(),
                                                    );
                                                    // Controls button
                                                    controls_button(
                                                        top_block_with_video_audio_and_other_buttons,
                                                        &localization_res,
                                                        fonts.button_font.clone(),
                                                    );
                                                },
                                            );
                                        main_settings_block_with_all_controls.spawn((
                                            // Middle block with settings
                                            Node {
                                                display: Display::Flex,
                                                flex_direction: FlexDirection::Column,
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(82.0),
                                                justify_content: JustifyContent::FlexStart,
                                                align_items: AlignItems::Center,
                                                // bottom: Val::Px(3.0),
                                                ..default()
                                            },
                                            Outline {
                                                color: Color::srgb_from_array([0.9, 0.1, 0.1]),
                                                width: Val::Px(2.0),
                                                ..default()
                                            },
                                        )).with_children(|options_middle_block|{
                                            video_tab(options_middle_block);
                                            audio_tab(options_middle_block, settings, &localization_res, &fonts);
                                        });
                                        main_settings_block_with_all_controls
                                            .spawn((
                                                // Lower block with buttons Apply and Back
                                                Node {
                                                    display: Display::Flex,
                                                    flex_direction: FlexDirection::Row,
                                                    width: Val::Percent(35.0),
                                                    height: Val::Percent(8.0),
                                                    justify_content: JustifyContent::SpaceBetween,
                                                    align_items: AlignItems::Center,
                                                    // bottom: Val::Px(3.0),
                                                    ..default()
                                                },
                                                Outline {
                                                    color: Color::srgb_from_array([0.4, 0.3, 0.8]),
                                                    width: Val::Px(2.0),
                                                    ..default()
                                                },
                                            ))
                                            .with_children(|bottom_buttons_block| {
                                                // Node for Apply button
                                                apply_button(
                                                    bottom_buttons_block,
                                                    &localization_res,
                                                    fonts.button_font.clone(),
                                                    options_images
                                                        .apply_and_back_button_image
                                                        .clone(),
                                                    OptionsButtonAction::Apply,
                                                );
                                                // Node for Back button
                                                back_button(
                                                    bottom_buttons_block,
                                                    &localization_res,
                                                    fonts.button_font.clone(),
                                                    options_images
                                                        .apply_and_back_button_image
                                                        .clone(),
                                                    OptionsButtonAction::Back,
                                                );
                                            });
                                    });
                            });
                    });
            });
    }
}
