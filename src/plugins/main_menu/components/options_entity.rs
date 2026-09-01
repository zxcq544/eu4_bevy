use crate::plugins::main_menu::resources::main_menu_all_images::MainMenuAllImages;
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;

#[derive(Component, Clone, Default)]
pub struct OptionsEntity;

impl OptionsEntity {
    pub fn as_scene_list(
        // main_menu_all_images_res: &Res<MainMenuAllImages>,
        localization_res: &Res<Localization>,
        fonts: &Res<FontHandles>,
    ) -> impl SceneList {
        // let button_font = fonts.button_font.clone();
        bsn_list! {
            OptionsEntity
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                // background_color: Color::TRANSPARENT,
                // overflow: Overflow::hidden(),
            }
            // BackgroundColor(Color::NONE)
            Children [
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // bottom: Val::Px(3.0),
                }
                Outline {
                    color: Color::srgb_from_array([0.4, 0.7, 0.5]),
                    width: Val::Px(2.0),
                }
                Children [
                ]
            ]
        }
    }
}
