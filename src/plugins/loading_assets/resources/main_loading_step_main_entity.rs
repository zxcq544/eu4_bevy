use crate::plugins::loading_assets::{
    loading_assets::MainLoadingStepBackgroundImage,
    resources::loading_screen_tooltip_image::LoadingScreenTooltipImage,
};
use bevy::{prelude::*, text::FontSourceTemplate};
use fonts::FontHandles;

#[derive(Component, Clone, Default)]
pub struct MainLoadingStepMainEntity;

impl MainLoadingStepMainEntity {
    pub fn as_scene_list(
        main_image_res: Res<MainLoadingStepBackgroundImage>,
        loading_screen_tooltip_image_res: Res<LoadingScreenTooltipImage>,
        fonts_res: Res<FontHandles>,
    ) -> impl SceneList {
        let main_image = main_image_res.image.clone();
        let loading_screen_tooltip_image = loading_screen_tooltip_image_res.image.clone();
        let loading_screen_tooltip_font = fonts_res.loading_screen_tooltip_font.clone();
        let loading_screen_loading_text_font = fonts_res.loading_screen_loading_text_font.clone();
        bsn_list! {
            // Main background image
            MainLoadingStepMainEntity
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::hidden(),
            }
            Children [
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                }
                ImageNode {
                    image: main_image,
                }
            ],
            // Text on top of screen with current loading info
            MainLoadingStepMainEntity
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                overflow: Overflow::hidden(),
            }
            Children [
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Vh(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    overflow: Overflow::hidden(),
                }
                Text::new("Loading")
                TextFont {
                    font: FontSourceTemplate::Handle(loading_screen_loading_text_font),
                }
                TextLayout {
                    justify: Justify::Center,
                }
                // Outline {
                //     color: Color::WHITE,
                //     width: Val::Px(1.0),
                // }
            ],
            // Image for tooltip at the bottom of screen
            MainLoadingStepMainEntity
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
                    width: Val::Vw(60.0),
                    height: Val::Vh(15.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                }
                ImageNode {
                    image: loading_screen_tooltip_image,
                    image_mode: NodeImageMode::Stretch,
                }
                // Outline {
                //     color: Color::BLACK,
                //     width: Val::Px(1.0),
                // }
                // Tooltip text
                Children [
                    Node {
                        width: Val::Percent(75.0),
                        height: Val::Percent(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        overflow: Overflow::hidden(),
                    }
                    Text::new("Tooltip")
                    TextFont {
                        font: FontSourceTemplate::Handle(loading_screen_tooltip_font),
                    }
                    TextLayout {
                        justify: Justify::Start,
                    }
                    // Outline {
                    //     color: Color::WHITE,
                    //     width: Val::Px(1.0),
                    // }
                ]
            ],
        }
    }
}
