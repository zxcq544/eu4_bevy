use crate::plugins::main_menu::resources::main_menu_all_images::MainMenuAllImages;
use bevy::{prelude::*, text::FontSourceTemplate};
use bevy_fluent::Localization;
use fluent_content::Content;
use fonts::FontHandles;

// Marker used for main menu button actions
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

// Marker for all main menu buttons to add hover effect and other effects
#[derive(Component, Clone, Default)]
pub struct MainMenuButton;

#[derive(Component, Clone, Default)]
pub struct MainMenuEntity;

impl MainMenuEntity {
    pub fn as_scene_list(
        main_menu_all_images_res: &Res<MainMenuAllImages>,
        localization_res: &Res<Localization>,
        fonts: &Res<FontHandles>,
    ) -> impl SceneList {
        let button_font = fonts.button_font.clone();
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
                Outline {
                    color: Color::srgb_from_array([0.3, 0.7, 0.2]),
                    width: Val::Px(2.0),
                }
                Children [
                    // Background image for continue button.
                    // background_image_for_continue_button(main_menu_all_images_res.continue_background_image.clone()),
                    // Background image for buttons single player, miltiplayer etc.
                    background_image_for_most_buttons(main_menu_all_images_res.main_menu_background_image.clone())
                    Children [
                        // Single player and multiplayer buttons block
                        Node {
                            display: Display::Flex,
                            width: Val::Percent(60.0),
                            height: Val::Percent(42.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            top: Val::Percent(16.0),
                        }
                        Children [
                            // Single player button
                            single_player_button(
                                &localization_res,
                                main_menu_all_images_res.single_player_button_image.clone(),
                                MainMenuButtonAction::SinglePlayer,
                                button_font.clone()
                            ),
                            // Multiplayer Button
                            multiplayer_button(
                                &localization_res,
                                main_menu_all_images_res.multiplayer_button_image.clone(),
                                MainMenuButtonAction::Multiplayer,
                                button_font.clone()
                            ),
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
                            width: Val::Percent(84.0),
                            height: Val::Percent(38.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            // overflow: Overflow::hidden(),
                            top: Val::Percent(17.0),
                        }
                        Children [
                            // Tutorial button
                            tutorial_button(
                                &localization_res,
                                main_menu_all_images_res.bg_image_lower_panel_main_menu_left_button.clone(),
                                MainMenuButtonAction::Tutorial,
                                button_font.clone()
                            ),
                            // Credits button
                            credits_button(
                                 &localization_res,
                                 main_menu_all_images_res.bg_image_lower_panel_main_menu_center_button.clone(),
                                 MainMenuButtonAction::Credits,
                                 button_font.clone()
                            ),
                            // Options button
                            options_button(
                                &localization_res,
                                main_menu_all_images_res.bg_image_lower_panel_main_menu_center_button.clone(),
                                MainMenuButtonAction::Options,
                                button_font.clone()
                            ),
                            // Exit button
                            exit_button(
                                &localization_res,
                                main_menu_all_images_res.bg_image_lower_panel_main_menu_right_button.clone(),
                                MainMenuButtonAction::Quit,
                                button_font.clone()
                            ),
                        ],
                    ]
                ]
            ]
        }
    }
}

pub fn tutorial_button(
    localization_res: &Res<Localization>,
    image: Handle<Image>,
    action_enum: MainMenuButtonAction,
    font: Handle<Font>,
) -> impl Scene {
    let label = localization_res.content("tutorial").expect(&format!(
        "missing tutorial in localisation files {:?}",
        localization_res
    ));
    bsn! {
        // Tutorial button
        Button
        MainMenuButton
        template_value(action_enum) // Workaround because bevy 0.19 doesn't support enum inside bsn!
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

        // Example but it is wrong i think
        // on(|_event: On<Pointer<Over>>, mut query: Query<(&mut MainMenuButton, &mut ImageNode)>, mut input_focus: ResMut<InputFocus>| {
        //     let entity = _event.event_target();
        //     if let Ok((mut button, mut image_node)) = query.get_mut(entity) {
        //         input_focus.set(entity, FocusCause::Pressed);
        //         image_node.color = HOVERED_BUTTON;
        //         button.set_changed();
        //     }
        // })
        // on(|_event: On<Pointer<Release>>, mut query: Query<(&mut MainMenuButton, &mut ImageNode)>, mut input_focus: ResMut<InputFocus>| {
        //     if let Ok((mut button, mut image_node)) = query.get_mut(_event.event_target()) {
        //         input_focus.clear();
        //         image_node.color = HOVERED_BUTTON;
        //         button.set_changed();
        //     }
        // })
        // on(|_event: On<Pointer<Press>>, mut query: Query<(&mut MainMenuButton, &mut ImageNode)>, mut input_focus: ResMut<InputFocus>| {
        //     let entity = _event.event_target();
        //     if let Ok((mut button, mut image_node)) = query.get_mut(entity) {
        //         input_focus.set(entity, FocusCause::Pressed);
        //         image_node.color = PRESSED_BUTTON;
        //         button.set_changed();
        //     }
        // })
        // on(|_event: On<Pointer<Out>>, mut query: Query<(&mut MainMenuButton, &mut ImageNode)>, mut input_focus: ResMut<InputFocus>| {
        //     if let Ok((mut button, mut image_node)) = query.get_mut(_event.event_target()) {
        //         input_focus.clear();
        //         image_node.color = NORMAL_BUTTON;
        //         button.set_changed();
        //     }
        // })
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
                    font: FontSourceTemplate::Handle(font),
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

pub fn credits_button(
    localization_res: &Res<Localization>,
    image: Handle<Image>,
    action_enum: MainMenuButtonAction,
    font: Handle<Font>,
) -> impl Scene {
    let label = localization_res.content("credits").expect(&format!(
        "missing credits in localisation files {:?}",
        localization_res
    ));
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
                    font: FontSourceTemplate::Handle(font),
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

pub fn options_button(
    localization_res: &Res<Localization>,
    image: Handle<Image>,
    action_enum: MainMenuButtonAction,
    font: Handle<Font>,
) -> impl Scene {
    let label = localization_res.content("options").expect(&format!(
        "missing options in localisation files {:?}",
        localization_res
    ));
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
                    font: FontSourceTemplate::Handle(font),
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
pub fn exit_button(
    localization_res: &Res<Localization>,
    image: Handle<Image>,
    action_enum: MainMenuButtonAction,
    font: Handle<Font>,
) -> impl Scene {
    let label = localization_res.content("exit").expect(&format!(
        "missing exit in localisation files {:?}",
        localization_res
    ));
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
                    font: FontSourceTemplate::Handle(font),
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

fn single_player_button(
    localization_res: &Res<Localization>,
    image: Handle<Image>,
    action_enum: MainMenuButtonAction,
    font: Handle<Font>,
) -> impl Scene {
    let label = localization_res.content("single_player").expect(&format!(
        "missing single_player in localisation files {:?}",
        localization_res
    ));
    bsn! {
        // Single player button
        Button
        MainMenuButton
        template_value(action_enum)
        Node {
            display: Display::Flex,
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        ImageNode {
            image: image,
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
            Children [
                Text::new(label)
                TextFont {
                    font_size: FontSize::Px(14.0),
                    font: FontSourceTemplate::Handle(font),
                }
                TextLayout {
                    justify: Justify::Center,
                }
            ]
        ]
        // Outline {
        //     color: Color::WHITE,
        //     width: Val::Px(1.0),
        // }
    }
}

pub fn multiplayer_button(
    localization_res: &Res<Localization>,
    image: Handle<Image>,
    action_enum: MainMenuButtonAction,
    font: Handle<Font>,
) -> impl Scene {
    let label = localization_res.content("multiplayer").expect(&format!(
        "missing multiplayer in localisation files {:?}",
        localization_res
    ));
    bsn! {
        // Multiplayer Button
        Button
        MainMenuButton
        template_value(action_enum)
        Node {
            display: Display::Flex,
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            overflow: Overflow::hidden(),
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
                right: Val::Percent(5.0),
            }
            Children [
                Text::new(label)
                TextFont {
                    font_size: FontSize::Px(14.0),
                    font: FontSourceTemplate::Handle(font),
                }
                TextLayout {
                    justify: Justify::Center,
                }
            ]
        ]
        // Outline {
        //     color: Color::srgb_from_array([0.2, 0.7, 0.7]),
        //     width: Val::Px(2.0),
        // }
    }
}

fn background_image_for_most_buttons(image: Handle<Image>) -> impl Scene {
    bsn! {
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
            image: image,
            image_mode: NodeImageMode::Stretch,
        }
        // Outline {
        //     color: Color::WHITE,
        //     width: Val::Px(1.0),
        // }
    }
}
