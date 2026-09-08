use crate::plugins::options::resources::options_images::OptionsImages;
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*, text::FontSourceTemplate};
use bevy_fluent::Localization;
use fluent_content::Content;
use fonts::FontHandles;

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
pub struct OptionsButtonsTopAndBottomRows;

impl OptionsButtonsTopAndBottomRows {
    pub fn spawn_using_commands(
        mut commands: Commands,
        localization_res: &Res<Localization>,
        fonts: &Res<FontHandles>,
        options_images: &Res<OptionsImages>,
    ) {
        commands
            .spawn((
                OptionsButtonsTopAndBottomRows,
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
                                        ));
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
                                                // back_button(
                                                //     &localization_res,
                                                //     fonts.button_font.clone(),
                                                //     options_images
                                                //         .apply_and_back_button_image
                                                //         .clone(),
                                                //     OptionsButtonAction::Back,
                                                // ),
                                            });
                                    });
                            });
                    });
            });
    }
}

fn apply_button(
    bottom_buttons_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    localization_res: &Res<Localization>,
    font: Handle<Font>,
    image: Handle<Image>,
    action_enum: OptionsButtonAction,
) {
    let label = localization_res.content("apply").expect(&format!(
        "missing apply in localisation files {:?}",
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
                color: Color::srgb_from_array([0.7, 0.2, 0.5]),
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

fn back_button(
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

fn video_button(
    top_block_with_video_audio_and_other_buttons: &mut RelatedSpawnerCommands<'_, ChildOf>,
    localization_res: &Res<Localization>,
    font: Handle<Font>,
) {
    let label = localization_res.content("video").expect(&format!(
        "missing video in localisation files {:?}",
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

fn audio_button(
    top_block_with_video_audio_and_other_buttons: &mut RelatedSpawnerCommands<'_, ChildOf>,
    localization_res: &Res<Localization>,
    font: Handle<Font>,
) {
    let label = localization_res.content("audio").expect(&format!(
        "missing audio in localisation files {:?}",
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

fn game_button(
    top_block_with_video_audio_and_other_buttons: &mut RelatedSpawnerCommands<'_, ChildOf>,
    localization_res: &Res<Localization>,
    font: Handle<Font>,
) {
    let label = localization_res.content("game").expect(&format!(
        "missing game in localisation files {:?}",
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

fn controls_button(
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