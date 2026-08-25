use crate::plugins::main_menu::resources::{
    background_image_for_continue::BackgroundImageForContinue,
    background_image_of_main_menu::BackgroundImageOfMainMenu,
    bg_image_lower_panel_main_menu_center_button::BgImageLowerPanelMainMenuCenterButton,
    bg_image_lower_panel_main_menu_left_button::BgImageLowerPanelMainMenuLeftButton,
    main_menu_multiplayer_button_image::MainMenuMultiplayerButtonImage,
    main_menu_single_player_button_image::MainMenuSinglePlayerButtonImage,
};
use bevy::prelude::*;

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
                    height: Val::Percent(20.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    // bottom: Val::Px(3.0),
                }
                Children [
                    // Background image for continue button
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(15.0),
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
                        width: Val::Percent(25.0),
                        height: Val::Percent(50.0),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        // overflow: Overflow::hidden(),
                    }
                    ImageNode {
                        image: background_image,
                        image_mode: NodeImageMode::Stretch,
                    }
                    Outline {
                        color: Color::WHITE,
                        width: Val::Px(1.0),
                    }
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
                            top: Val::Percent(20.0),
                        }
                        Children [
                            // Tutorial button
                            Node {
                                display: Display::Flex,
                                width: Val::Percent(25.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                            }
                            ImageNode {
                                image: bg_image_lower_panel_main_menu_left_button,
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
                                    Text::new("Tutorial")
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
                            ],
                            // Credits button
                            Node {
                                display: Display::Flex,
                                width: Val::Percent(25.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                            }
                            ImageNode {
                                image: bg_image_lower_panel_main_menu_center_button_first,
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
                                    Text::new("Credits")
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
                            ],
                            // Outline {
                            //     color: Color::srgb_from_array([0.2, 0.7, 0.7]),
                            //     width: Val::Px(2.0),
                            // },
                            // Options button
                            Node {
                                display: Display::Flex,
                                width: Val::Percent(25.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                            }
                            ImageNode {
                                image: bg_image_lower_panel_main_menu_center_button_second,
                                image_mode: NodeImageMode::Stretch,
                            }
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
                                    Text::new("Options")
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
                            ],
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
