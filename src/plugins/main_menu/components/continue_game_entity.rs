use crate::plugins::main_menu::{
    components::main_menu_entity::{MainMenuButton, MainMenuButtonAction},
    resources::main_menu_all_images::MainMenuAllImages,
};
use bevy::{prelude::*, text::FontSourceTemplate};
use bevy_fluent::Localization;
use fluent_content::Content;
use fonts::FontHandles;

#[derive(Component, Clone, Default)]
pub struct ContinueGameEntity;

impl ContinueGameEntity {
    pub fn as_scene_list(
        main_menu_all_images_res: &Res<MainMenuAllImages>,
        localization_res: &Res<Localization>,
        fonts: &Res<FontHandles>,
    ) -> impl SceneList {
        let button_font = fonts.button_font.clone();
        bsn_list! {
            ContinueGameEntity
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                // overflow: Overflow::hidden(),
            }
            ZIndex(3)
            Children [
                // Background image for continue button.
                background_image_for_continue_button(main_menu_all_images_res.continue_background_image.clone())
                Children [
                    // Continue button
                    continue_button(
                        &localization_res,
                        main_menu_all_images_res.continue_button_image.clone(),
                        MainMenuButtonAction::Continue,
                        button_font.clone()
                    )
                ]
                // Outline {
                //     color: Color::srgb_from_array([0.3, 0.7, 0.3]),
                //     width: Val::Px(2.0),
            ]
        }
    }
}

fn background_image_for_continue_button(image: Handle<Image>) -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            // flex_direction: FlexDirection::Column,
            width: Val::Percent(20.0),
            height: Val::Percent(7.6),
            bottom: Val::Percent(10.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            // top: Val::Percent(10.0),
            // z-index: 1,
            // overflow: Overflow::hidden(),
        }
        ZIndex(3)
        ImageNode {
            image: image,
            image_mode: NodeImageMode::Stretch,
        }
        // Outline {
        //     color: Color::BLACK,
        //     width: Val::Px(2.0),
        // }
    }
}

fn continue_button(
    localization_res: &Res<Localization>,
    image: Handle<Image>,
    action_enum: MainMenuButtonAction,
    font: Handle<Font>,
) -> impl Scene {
    let label = localization_res.content("continue").expect(&format!(
        "missing continue in localisation files {:?}",
        localization_res
    ));
    bsn! {
        // Continue button
        Button
        MainMenuButton
        template_value(action_enum)
        Node {
            display: Display::Flex,
            width: Val::Percent(79.0),
            height: Val::Percent(64.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            bottom: Val::Percent(8.0),
        }
        ZIndex(4)
        ImageNode {
            image: image,
            image_mode: NodeImageMode::Stretch,
        }
        // Outline {
        //     color: Color::srgb_from_array([0.3, 0.7, 0.5]),
        //     width: Val::Px(2.0),
        // }
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
            // }
        ]
    }
}
