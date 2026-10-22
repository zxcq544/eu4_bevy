use crate::{
    plugins::main_menu::{
        components::continue_game_entity::ContinueGameEntity,
        resources::main_menu_all_images::MainMenuAllImages,
        systems::main_menu_button_system_united::{
            on_main_menu_button_click, on_main_menu_button_down, on_main_menu_button_hover,
            on_main_menu_button_unhover,
        },
    },
    shared::fonts::fonts::FontHandles,
};
use bevy::{prelude::*, ui::widget::ImageNode};
use bevy_fluent::Localization;
use fluent_content::Content;

// ── Markers ──────────────────────────────────────────────────────────
#[derive(Component, Clone, Default)]
pub enum MainMenuButtonAction {
    #[default]
    NoAction,
    SinglePlayer,
    Multiplayer,
    Continue,
    Tutorial,
    Credits,
    Options,
    Quit,
}

#[derive(Component, Clone, Default)]
pub struct MainMenuButton;

#[derive(Component, Clone, Default)]
pub struct MainMenuEntity;

// ── Main menu root ───────────────────────────────────────────────────
pub fn spawn_main_menu(
    commands: &mut Commands,
    images: &MainMenuAllImages,
    localization: &Localization,
    fonts: &FontHandles,
) {
    let button_font = fonts.button_font.clone();

    commands
        .spawn((
            MainMenuEntity,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                ..default()
            },
            // Visibility::Hidden,
            BackgroundColor(Color::NONE),
            // ZIndex(1),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(25.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Outline {
                    color: Color::srgb_from_array([0.3, 0.7, 0.2]),
                    width: Val::Px(2.0),
                    ..default()
                },
            ))
            .with_children(|lower_panel| {
                // Continue button block
                lower_panel
                    .spawn((
                        ContinueGameEntity,
                        Node {
                            display: Display::None,
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(20.0),
                            height: Val::Percent(35.0),
                            justify_content: JustifyContent::FlexEnd,
                            align_items: AlignItems::Center,
                            top: Val::Percent(10.0),
                            ..default()
                        },
                        ZIndex(2),
                        // Outline {
                        //     color: Color::WHITE,
                        //     width: Val::Px(2.0),
                        //     ..default()
                        // },
                        ImageNode::new(images.continue_background_image.clone())
                            .with_mode(NodeImageMode::Stretch),
                    ))
                    .with_children(|continue_button_block| {
                        spawn_continue_button(
                            continue_button_block,
                            &localization,
                            images.continue_button_image.clone(),
                            MainMenuButtonAction::Continue,
                            button_font.clone(),
                        );
                    });
                // lower block with other buttons
                lower_panel
                    .spawn((
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(33.0),
                            height: Val::Percent(50.0),
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        // ZIndex(1),
                        ImageNode::new(images.main_menu_background_image.clone())
                            .with_mode(NodeImageMode::Stretch),
                    ))
                    .with_children(|button_panel| {
                        // ── Single‑player + multiplayer row ───────────
                        button_panel
                            .spawn((
                                Node {
                                    display: Display::Flex,
                                    width: Val::Percent(60.0),
                                    height: Val::Percent(42.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    top: Val::Percent(16.0),
                                    ..default()
                                },
                                // ZIndex(1),
                            ))
                            .with_children(|top_row| {
                                spawn_single_player_button(
                                    top_row,
                                    localization,
                                    images.single_player_button_image.clone(),
                                    MainMenuButtonAction::SinglePlayer,
                                    button_font.clone(),
                                );
                                spawn_multiplayer_button(
                                    top_row,
                                    localization,
                                    images.multiplayer_button_image.clone(),
                                    MainMenuButtonAction::Multiplayer,
                                    button_font.clone(),
                                );
                            });

                        // ── Lower 4‑button row ─────────────────────────
                        button_panel
                            .spawn((Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                width: Val::Percent(84.0),
                                height: Val::Percent(38.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                top: Val::Percent(17.0),
                                ..default()
                            },))
                            .with_children(|bottom_row| {
                                spawn_tutorial_button(
                                    bottom_row,
                                    localization,
                                    images.bg_image_lower_panel_main_menu_left_button.clone(),
                                    MainMenuButtonAction::Tutorial,
                                    button_font.clone(),
                                );
                                spawn_credits_button(
                                    bottom_row,
                                    localization,
                                    images.bg_image_lower_panel_main_menu_center_button.clone(),
                                    MainMenuButtonAction::Credits,
                                    button_font.clone(),
                                );
                                spawn_options_button(
                                    bottom_row,
                                    localization,
                                    images.bg_image_lower_panel_main_menu_center_button.clone(),
                                    MainMenuButtonAction::Options,
                                    button_font.clone(),
                                );
                                spawn_exit_button(
                                    bottom_row,
                                    localization,
                                    images.bg_image_lower_panel_main_menu_right_button.clone(),
                                    MainMenuButtonAction::Quit,
                                    button_font.clone(),
                                );
                            });
                    });
            });
        });
}

// ── Individual button helpers ────────────────────────────────────────
fn spawn_tutorial_button(
    commands: &mut ChildSpawnerCommands,
    localization: &Localization,
    image: Handle<Image>,
    action: MainMenuButtonAction,
    font: Handle<Font>,
) {
    let label = localization
        .content("tutorial")
        .expect("missing tutorial in localisation files");

    commands
        .spawn((
            Button,
            MainMenuButton,
            action,
            Node {
                display: Display::Flex,
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            // ZIndex(1),
            ImageNode::new(image).with_mode(NodeImageMode::Stretch),
        ))
        .with_children(|button| {
            button
                .spawn((Node {
                    display: Display::Flex,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    left: Val::Percent(8.0),
                    ..default()
                },))
                .with_children(|text_container| {
                    text_container.spawn((
                        Text::new(label),
                        TextFont {
                            font: font.into(),
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            ..default()
                        },
                    ));
                });
        })
        .observe(on_main_menu_button_hover)
        .observe(on_main_menu_button_unhover)
        .observe(on_main_menu_button_down)
        .observe(on_main_menu_button_click);
}

fn spawn_credits_button(
    commands: &mut ChildSpawnerCommands,
    localization: &Localization,
    image: Handle<Image>,
    action: MainMenuButtonAction,
    font: Handle<Font>,
) {
    let label = localization
        .content("credits")
        .expect("missing credits in localisation files");

    commands
        .spawn((
            Button,
            MainMenuButton,
            action,
            Node {
                display: Display::Flex,
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::new(image).with_mode(NodeImageMode::Stretch),
        ))
        .with_children(|button| {
            button
                .spawn(Node {
                    display: Display::Flex,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|text_container| {
                    text_container.spawn((
                        Text::new(label),
                        TextFont {
                            font: font.into(),
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            ..default()
                        },
                    ));
                });
        })
        .observe(on_main_menu_button_hover)
        .observe(on_main_menu_button_unhover)
        .observe(on_main_menu_button_down)
        .observe(on_main_menu_button_click);
}

fn spawn_options_button(
    commands: &mut ChildSpawnerCommands,
    localization: &Localization,
    image: Handle<Image>,
    action: MainMenuButtonAction,
    font: Handle<Font>,
) {
    let label = localization
        .content("options")
        .expect("missing options in localisation files");

    commands
        .spawn((
            Button,
            MainMenuButton,
            action,
            Node {
                display: Display::Flex,
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::new(image).with_mode(NodeImageMode::Stretch),
        ))
        .with_children(|button| {
            button
                .spawn(Node {
                    display: Display::Flex,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|text_container| {
                    text_container.spawn((
                        Text::new(label),
                        TextFont {
                            font: font.into(),
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            ..default()
                        },
                    ));
                });
        })
        .observe(on_main_menu_button_hover)
        .observe(on_main_menu_button_unhover)
        .observe(on_main_menu_button_down)
        .observe(on_main_menu_button_click);
}

fn spawn_exit_button(
    commands: &mut ChildSpawnerCommands,
    localization: &Localization,
    image: Handle<Image>,
    action: MainMenuButtonAction,
    font: Handle<Font>,
) {
    let label = localization
        .content("exit")
        .expect("missing exit in localisation files");

    commands
        .spawn((
            Button,
            MainMenuButton,
            action,
            Node {
                display: Display::Flex,
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::new(image).with_mode(NodeImageMode::Stretch),
        ))
        .with_children(|button| {
            button
                .spawn(Node {
                    display: Display::Flex,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    right: Val::Percent(5.0),
                    ..default()
                })
                .with_children(|text_container| {
                    text_container.spawn((
                        Text::new(label),
                        TextFont {
                            font: font.into(),
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            ..default()
                        },
                    ));
                });
        })
        .observe(on_main_menu_button_hover)
        .observe(on_main_menu_button_unhover)
        .observe(on_main_menu_button_down)
        .observe(on_main_menu_button_click);
}

fn spawn_single_player_button(
    commands: &mut ChildSpawnerCommands,
    localization: &Localization,
    image: Handle<Image>,
    action: MainMenuButtonAction,
    font: Handle<Font>,
) {
    let label = localization
        .content("single_player")
        .expect("missing single_player in localisation files");

    commands
        .spawn((
            Button,
            MainMenuButton,
            action,
            Node {
                display: Display::Flex,
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::new(image).with_mode(NodeImageMode::Stretch),
        ))
        .with_children(|button| {
            button
                .spawn(Node {
                    display: Display::Flex,
                    width: Val::Percent(90.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    left: Val::Percent(5.0),
                    ..default()
                })
                .with_children(|text_container| {
                    text_container.spawn((
                        Text::new(label),
                        TextFont {
                            font: font.into(),
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            ..default()
                        },
                    ));
                });
        })
        .observe(on_main_menu_button_hover)
        .observe(on_main_menu_button_unhover)
        .observe(on_main_menu_button_down)
        .observe(on_main_menu_button_click);
}

fn spawn_multiplayer_button(
    commands: &mut ChildSpawnerCommands,
    localization: &Localization,
    image: Handle<Image>,
    action: MainMenuButtonAction,
    font: Handle<Font>,
) {
    let label = localization
        .content("multiplayer")
        .expect("missing multiplayer in localisation files");

    commands
        .spawn((
            Button,
            MainMenuButton,
            action,
            Node {
                display: Display::Flex,
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::hidden(),
                ..default()
            },
            ImageNode::new(image).with_mode(NodeImageMode::Stretch),
        ))
        .with_children(|button| {
            button
                .spawn(Node {
                    display: Display::Flex,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    right: Val::Percent(5.0),
                    ..default()
                })
                .with_children(|text_container| {
                    text_container.spawn((
                        Text::new(label),
                        TextFont {
                            font: font.into(),
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            ..default()
                        },
                    ));
                });
        })
        .observe(on_main_menu_button_hover)
        .observe(on_main_menu_button_unhover)
        .observe(on_main_menu_button_down)
        .observe(on_main_menu_button_click);
}

fn spawn_continue_button(
    commands: &mut ChildSpawnerCommands,
    localization: &Localization,
    image: Handle<Image>,
    action: MainMenuButtonAction,
    font: Handle<Font>,
) {
    let label = localization
        .content("continue")
        .expect("missing continue in localisation files");

    commands
        .spawn((
            Button,
            MainMenuButton,
            action,
            Node {
                display: Display::Flex,
                width: Val::Percent(79.0),
                height: Val::Percent(64.0),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                bottom: Val::Percent(9.0),
                ..default()
            },
            ImageNode::new(image).with_mode(NodeImageMode::Stretch),
        ))
        .with_children(|button| {
            button
                .spawn((Node {
                    display: Display::Flex,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|text_container| {
                    text_container.spawn((
                        Text::new(label),
                        TextFont {
                            font: font.into(),
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            ..default()
                        },
                    ));
                });
        })
        .observe(on_main_menu_button_hover)
        .observe(on_main_menu_button_unhover)
        .observe(on_main_menu_button_down)
        .observe(on_main_menu_button_click);
}
