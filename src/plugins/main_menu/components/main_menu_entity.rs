use crate::plugins::main_menu::resources::{
    background_image_for_continue::BackgroundImageForContinue,
    background_image_of_main_menu::BackgroundImageOfMainMenu,
    bg_image_lower_panel_main_menu_center_button::BgImageLowerPanelMainMenuCenterButton,
    bg_image_lower_panel_main_menu_left_button::BgImageLowerPanelMainMenuLeftButton,
    bg_image_lower_panel_main_menu_right_button::BgImageLowerPanelMainMenuRightButton,
    main_menu_multiplayer_button_image::MainMenuMultiplayerButtonImage,
    main_menu_single_player_button_image::MainMenuSinglePlayerButtonImage,
};
use bevy::{
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};

// Marker used for main menu button actions
#[derive(Component, Clone, Default)]
pub enum MainMenuButtonAction {
    #[default]
    NoAction,
    Tutorial,
    Credits,
    Options,
    Quit,
}

// Marker for all main menu buttons to add hover effect and other effects
#[derive(Component, Clone, Default)]
pub struct MainMenuButton;

#[derive(Component, Clone, Default)]
pub struct MainMenuEntity;

impl MainMenuEntity {
    pub fn as_scene_list(
        background_image_res: Res<BackgroundImageOfMainMenu>,
        continue_background_image_res: Res<BackgroundImageForContinue>,
        single_player_button_image_res: Res<MainMenuSinglePlayerButtonImage>,
        multiplayer_button_image_res: Res<MainMenuMultiplayerButtonImage>,
        bg_image_lower_panel_main_menu_left_button_res: Res<BgImageLowerPanelMainMenuLeftButton>,
        bg_image_lower_panel_main_menu_center_button_res: Res<
            BgImageLowerPanelMainMenuCenterButton,
        >,
        bg_image_lower_panel_main_menu_right_button_res: Res<BgImageLowerPanelMainMenuRightButton>,
    ) -> impl SceneList {
        let background_image = background_image_res.image.clone();
        let continue_background_image = continue_background_image_res.image.clone();
        let single_player_button_image = single_player_button_image_res.image.clone();
        let multiplayer_button_image = multiplayer_button_image_res.image.clone();
        let bg_image_lower_panel_main_menu_left_button =
            bg_image_lower_panel_main_menu_left_button_res.image.clone();
        let bg_image_lower_panel_main_menu_center_button_first =
            bg_image_lower_panel_main_menu_center_button_res
                .image
                .clone();
        let bg_image_lower_panel_main_menu_center_button_second =
            bg_image_lower_panel_main_menu_center_button_res
                .image
                .clone();
        let bg_image_lower_panel_main_menu_right_button =
            bg_image_lower_panel_main_menu_right_button_res
                .image
                .clone();
        bsn_list! {
            MainMenuEntity
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                overflow: Overflow::hidden(),
            }
            Children [
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(25.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    // bottom: Val::Px(3.0),
                }
                Children [
                    // Background image for continue button
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(20.0),
                        height: Val::Percent(30.0),
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::End,
                        bottom: Val::Percent(-10.0),
                        // z-index: 1,
                        // overflow: Overflow::hidden(),
                    }
                    ImageNode {
                        image: continue_background_image,
                        image_mode: NodeImageMode::Stretch,
                    }
                    // Outline {
                    //     color: Color::BLACK,
                    //     width: Val::Px(2.0),
                    // }
                    ZIndex(2),
                    // Background image for buttons single player, miltiplayer etc.
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(33.0),
                        height: Val::Percent(50.0),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        // overflow: Overflow::hidden(),
                    }
                    ImageNode {
                        image: background_image,
                        image_mode: NodeImageMode::Stretch,
                    }
                    // Outline {
                    //     color: Color::WHITE,
                    //     width: Val::Px(1.0),
                    // }
                    Children [
                        // Single player and multiplayer buttons block
                        Node {
                            display: Display::Flex,
                            width: Val::Percent(60.0),
                            height: Val::Percent(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            top: Val::Percent(17.0),
                        }
                        Children [
                            // Single player button
                            Node {
                                display: Display::Flex,
                                width: Val::Percent(50.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                // overflow: Overflow::hidden(),
                            }
                            ImageNode {
                                image: single_player_button_image,
                                image_mode: NodeImageMode::Stretch,
                            }
                            Children [
                                // Text node
                                Node {
                                    display: Display::Flex,
                                    width: Val::Percent(90.0),
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    left: Val::Percent(5.0),
                                }
                                // Outline {
                                //     color: Color::WHITE,
                                //     width: Val::Px(1.0),
                                // }
                                Children [
                                    Text::new("Single Player")
                                    TextFont {
                                        font_size: FontSize::Px(14.0),
                                    }
                                    TextLayout {
                                        justify: Justify::Center,
                                    }
                                ]
                            ],
                            // Outline {
                            //     color: Color::srgb_from_array([0.2, 0.2, 0.7]),
                            //     width: Val::Px(2.0),
                            // }
                            // Multiplayer Button
                            Node {
                                display: Display::Flex,
                                width: Val::Percent(50.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                overflow: Overflow::hidden(),
                            }
                            ImageNode {
                                image: multiplayer_button_image,
                                image_mode: NodeImageMode::Stretch,
                            }
                            Children [
                                Node {
                                    display: Display::Flex,
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    right: Val::Percent(5.0),
                                }
                                Children [
                                    Text::new("Multiplayer")
                                    TextFont {
                                        font_size: FontSize::Px(14.0),
                                    }
                                    TextLayout {
                                        justify: Justify::Center,
                                    }
                                ]
                            ],
                            // Outline {
                            //     color: Color::srgb_from_array([0.2, 0.7, 0.7]),
                            //     width: Val::Px(2.0),
                            // }
                        ]
                        // Outline {
                        //     color: Color::srgb_from_array([0.7, 0.7, 0.2]), // yellow
                        //     width: Val::Px(2.0),
                        // }
                        ZIndex(2),
                        // 4 button lower block with buttons Tutorial, Credits, Options, Exit
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(80.0),
                            height: Val::Percent(35.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            // overflow: Overflow::hidden(),
                            top: Val::Percent(21.0),
                        }
                        Children [
                            // Tutorial button
                            main_menu_small_left_button(
                                "Tutorial".to_string(),
                                bg_image_lower_panel_main_menu_left_button,
                                MainMenuButtonAction::Tutorial
                            ),
                            // Credits button
                            main_menu_small_center_button(
                                "Credits".to_string(),
                                bg_image_lower_panel_main_menu_center_button_first,
                                MainMenuButtonAction::Credits
                            ),
                            // Options button
                            main_menu_small_center_button(
                                "Options".to_string(),
                                bg_image_lower_panel_main_menu_center_button_second,
                                MainMenuButtonAction::Options
                            ),
                            // Exit button
                            main_menu_small_right_button(
                                "Exit".to_string(),
                                bg_image_lower_panel_main_menu_right_button,
                                MainMenuButtonAction::Quit
                            ),
                        ],
                    ],
                ]
                // Outline {
                //     color: Color::srgb_from_array([0.3, 0.7, 0.2]),
                //     width: Val::Px(2.0),
                // }
            ]
        }
    }
}

pub fn main_menu_button_action(
    interaction_query: Query<
        (&Interaction, &MainMenuButtonAction),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MainMenuButtonAction::Quit => {
                    info!("Quit button pressed");
                    app_exit_writer.write(AppExit::Success);
                }
                _ => {}
            }
        }
    }
}

pub const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: Color = Color::srgb(1.2, 1.25, 1.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn main_menu_button_hover(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MainMenuButton, &mut ImageNode),
        Changed<Interaction>,
    >,
) {
    for (entity, interaction, mut button, mut image_node) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity, FocusCause::Pressed);
                image_node.color = PRESSED_BUTTON;
                // The accessibility system's only update the button's state when the `Button` component is marked as changed.
                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity, FocusCause::Pressed);
                image_node.color = HOVERED_BUTTON;
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                image_node.color = NORMAL_BUTTON;
                button.set_changed();
            }
        }
    }
}

pub fn main_menu_small_left_button(
    label: String,
    image: Handle<Image>,
    action_enum: MainMenuButtonAction,
) -> impl Scene {
    bsn! {
        // Tutorial button
        Button
        MainMenuButton
        template_value(action_enum)
        Node {
            display: Display::Flex,
            width: Val::Percent(25.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        ImageNode {
            image: image,
            image_mode: NodeImageMode::Stretch,
        }
        Children [
            Node {
                display: Display::Flex,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                left: Val::Percent(8.0),
            }
            Children [
                Text::new(label)
                TextFont {
                    font_size: FontSize::Px(14.0),
                }
                TextLayout {
                    justify: Justify::Center,
                }
            ]
            // Outline {
            //     color: Color::srgb_from_array([0.3, 0.7, 0.3]),
            //     width: Val::Px(2.0),
            // },
        ]
        // Outline {
        //     color: Color::srgb_from_array([0.3, 0.7, 0.3]),
        //     width: Val::Px(2.0),
        // },
    }
}

pub fn main_menu_small_center_button(
    label: String,
    image: Handle<Image>,
    action_enum: MainMenuButtonAction,
) -> impl Scene {
    bsn! {
        Button
        MainMenuButton
        template_value(action_enum)
        Node {
            display: Display::Flex,
            width: Val::Percent(25.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        ImageNode {
            image: image,
            image_mode: NodeImageMode::Stretch,
        }
        // Text node
        Children [
            Node {
                display: Display::Flex,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // left: Val::Percent(8.0),
            }
            Children [
                Text::new(label)
                TextFont {
                    font_size: FontSize::Px(14.0),
                }
                TextLayout {
                    justify: Justify::Center,
                }
            ]
            // Outline {
            //     color: Color::srgb_from_array([0.3, 0.7, 0.3]),
            //     width: Val::Px(2.0),
            // },
        ]
        // Outline {
        //     color: Color::srgb_from_array([0.3, 0.7, 0.3]),
        //     width: Val::Px(2.0),
        // },
    }
}

pub fn main_menu_small_right_button(
    label: String,
    image: Handle<Image>,
    action_enum: MainMenuButtonAction,
) -> impl Scene {
    bsn! {
        // Exit button
        Button
        MainMenuButton
        template_value(action_enum)
        Node {
            display: Display::Flex,
            width: Val::Percent(25.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        ImageNode {
            image: image,
            image_mode: NodeImageMode::Stretch,
            // color: Color::srgb_from_array([1.3, 1.7, 1.2]),
        }
        Children [
            Node {
                display: Display::Flex,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                right: Val::Percent(5.0),
            }
            Children [
                Text::new(label)
                TextFont {
                    font_size: FontSize::Px(14.0),
                }
                TextLayout {
                    justify: Justify::Center,
                }
            ]
            // Outline {
            //     color: Color::srgb_from_array([0.3, 0.7, 0.3]),
            //     width: Val::Px(2.0),
            // },
        ]
        // Outline {
        //     color: Color::srgb_from_array([0.3, 0.7, 0.3]),
        //     width: Val::Px(2.0),
        // },
    }
}
