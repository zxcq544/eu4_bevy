use crate::plugins::main_menu::resources::background_image_of_main_menu::BackgroundImageOfMainMenu;
use bevy::prelude::*;

#[derive(Component, Clone, Default)]
pub struct MainMenuEntity;

impl MainMenuEntity {
    pub fn as_scene_list(background_image_res: Res<BackgroundImageOfMainMenu>) -> impl SceneList {
        let background_image = background_image_res.image.clone();
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
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    overflow: Overflow::hidden(),
                }
                ImageNode {
                    image: background_image,
                }
                Outline {
                    color: Color::WHITE,
                    width: Val::Px(1.0),
                }
                // Node {
                //     width: Val::Percent(100.0),
                //     height: Val::Percent(100.0),
                //     position_type: PositionType::Absolute,
                //     justify_content: JustifyContent::Center,
                //     align_items: AlignItems::Center,
                //     overflow: Overflow::hidden(),
                // }
            ]
        }
    }
}
