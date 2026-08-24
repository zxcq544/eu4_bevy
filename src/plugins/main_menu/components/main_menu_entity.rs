use crate::plugins::main_menu::resources::{
    background_image_for_continue::BackgroundImageForContinue,
    background_image_of_main_menu::BackgroundImageOfMainMenu,
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
    ) -> impl SceneList {
        let background_image = background_image_res.image.clone();
        let continue_background_image = continue_background_image_res.image.clone();
        let single_player_button_image = single_player_button_image_res.image.clone();
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
                        width: Val::Percent(15.0),
                        height: Val::Percent(50.0),
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::End,
                        bottom: Val::Vw(-0.95),
                        // z-index: 1,
                        // overflow: Overflow::hidden(),
                    }
                    ImageNode {
                        image: continue_background_image,
                    }
                    // Outline {
                    //     color: Color::BLACK,
                    //     width: Val::Px(2.0),
                    // }
                    ZIndex(2),
                    // Background image for buttons single player, miltiplayer etc.
                    Node {
                        display: Display::Flex,
                        width: Val::Percent(25.0),
                        height: Val::Percent(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        overflow: Overflow::hidden(),
                    }
                    ImageNode {
                        image: background_image,
                    }
                    // Outline {
                    //     color: Color::WHITE,
                    //     width: Val::Px(1.0),
                    // }
                    Children [
                        // Single player button
                        Node {
                            display: Display::Flex,
                            width: Val::Percent(35.0),
                            height: Val::Percent(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            // overflow: Overflow::hidden(),
                            left: Val::Percent(-13.0),
                            top: Val::Percent(-13.0),
                        }
                        ImageNode {
                            image: single_player_button_image,
                            image_mode: NodeImageMode::Stretch,
                        }
                        Text::new("Single player")
                        TextFont{
                            font_size: FontSize::Px(14.0),
                        }
                        // Outline {
                        //     color: Color::srgb_from_array([0.7, 0.7, 0.2]),
                        //     width: Val::Px(2.0),
                        // }
                        ZIndex(2)
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
