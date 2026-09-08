use crate::plugins::options::resources::options_images::OptionsImages;
use bevy::prelude::*;

#[derive(Component, Clone, Default)]
pub struct OptionsBackgroundEntity;

impl OptionsBackgroundEntity {
    pub fn as_scene_list(options_images: &Res<OptionsImages>) -> impl SceneList {
        // let button_font = fonts.button_font.clone();
        let settings_bg_image = options_images.settings_bg_image.clone();
        bsn_list! {
            OptionsBackgroundEntity
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // background_color: Color::TRANSPARENT,
                // overflow: Overflow::hidden(),
            }
            Visibility::Hidden // Spawn as hidden because we don't despawn ingame ui elements
            ZIndex(1)
            BackgroundColor(Color::NONE)
            Children [
                // Options Background image node
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
                ImageNode {
                    image: settings_bg_image,
                    image_mode: NodeImageMode::Stretch,
                }
            ]
        }
    }
    pub fn spawn_using_commands(mut commands: Commands, options_images: Res<OptionsImages>) {
        commands
            .spawn((
                OptionsBackgroundEntity,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    // background_color: Color::TRANSPARENT,
                    // overflow: Overflow::hidden(),
                    align_items: AlignItems::Center,
                    ..default()
                },
                Visibility::Hidden, // Spawn as hidden because we don't despawn ingame ui elements
                ZIndex(1),
                BackgroundColor(Color::NONE),
            ))
            .with_children(|parent| {
                parent.spawn((
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
                    ImageNode {
                        image: options_images.settings_bg_image.clone(),
                        image_mode: NodeImageMode::Stretch,
                        ..default()
                    },
                ));
            });
    }
}
