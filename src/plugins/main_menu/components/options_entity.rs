use crate::plugins::main_menu::resources::options_images::OptionsImages;
use bevy::{prelude::*, text::FontSourceTemplate};
use bevy_fluent::Localization;
use fonts::FontHandles;

#[derive(Component, Clone, Default)]
pub struct OptionsEntity;

impl OptionsEntity {
    pub fn as_scene_list(
        options_images: &Res<OptionsImages>,
        localization_res: &Res<Localization>,
        fonts: &Res<FontHandles>,
    ) -> impl SceneList {
        // let button_font = fonts.button_font.clone();
        let settings_bg_image = options_images.settings_bg_image.clone();
        bsn_list! {
            OptionsEntity
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // background_color: Color::TRANSPARENT,
                // overflow: Overflow::hidden(),
            }
            ZIndex(10)
            BackgroundColor(Color::NONE)
            Children [
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(70.0),
                    height: Val::Vh(90.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // bottom: Val::Px(3.0),
                }
                Outline {
                    color: Color::srgb_from_array([0.4, 0.7, 0.5]),
                    width: Val::Px(2.0),
                }
                Children [
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        // bottom: Val::Px(3.0),
                    }
                    // Outline {
                    //     color: Color::srgb_from_array([0.4, 0.7, 0.5]),
                    //     width: Val::Px(2.0),
                    // }
                    ImageNode {
                        image: settings_bg_image,
                        image_mode: NodeImageMode::Stretch,
                    }
                    Children [
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
                        }
                        Outline {
                            color: Color::srgb_from_array([0.7, 0.2, 0.5]),
                            width: Val::Px(2.0),
                        }

                        // Two blocks. One with all controls, second with two buttons: Apply and Back
                        Children [
                            // Main settings block with all controls
                            Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                width: Val::Percent(100.0),
                                height: Val::Percent(95.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                // bottom: Val::Px(3.0),
                            },
                            // Outline {
                            //     color: Color::srgb_from_array([0.4, 0.9, 0.5]),
                            //     width: Val::Px(2.0),
                            // },
                            // Buttons block with Apply and Back
                            Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                width: Val::Percent(20.0),
                                height: Val::Percent(5.0),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                // bottom: Val::Px(3.0),
                            }
                            // Outline {
                            //     color: Color::srgb_from_array([0.4, 0.3, 0.8]),
                            //     width: Val::Px(2.0),
                            // }
                            Children [
                                // Node for Apply button
                                apply_button(fonts.button_font.clone()),
                                // Node for Back button
                                back_button(fonts.button_font.clone()),
                            ],
                        ]
                    ]
                ]
            ]
        }
    }
}

fn apply_button(font: Handle<Font>) -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(40.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            // bottom: Val::Px(3.0),
        }
        // Outline {
        //     color: Color::srgb_from_array([0.7, 0.2, 0.5]),
        //     width: Val::Px(2.0),
        // }
        Children [
            Text::new("Apply")
            TextFont {
                font_size: FontSize::Px(14.0),
                font: FontSourceTemplate::Handle(font),
            }
            TextLayout {
                justify: Justify::Center,
            }
        ]
    }
}

fn back_button(font: Handle<Font>) -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(40.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            // bottom: Val::Px(3.0),
        }
        // Outline {
        //     color: Color::srgb_from_array([0.2, 0.7, 0.2]),
        //     width: Val::Px(2.0),
        // }
        Children [
            Text::new("Back")
            TextFont {
                font_size: FontSize::Px(14.0),
                font: FontSourceTemplate::Handle(font),
            }
            TextLayout {
                justify: Justify::Center,
            }
        ]
    }
}
