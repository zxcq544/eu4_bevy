use crate::{
    core::states::GameState,
    plugins::loading_assets::{
        loading_assets::{MainLoadingStepBackgroundImage, MainLoadingStepMainEntity},
        resources::loading_screen_tooltip_image::LoadingScreenTooltipImage,
    },
};
use bevy::prelude::*;

pub fn set_background_images(
    commands: Commands,
    main_image: Res<MainLoadingStepBackgroundImage>,
    loading_screen_tooltip_image: Res<LoadingScreenTooltipImage>,
    current_state: Res<State<GameState>>,
) {
    set_main_loading_step_background_image(
        commands,
        main_image,
        loading_screen_tooltip_image,
        current_state,
    );
}

// TODO: try to bsn this function
fn set_main_loading_step_background_image(
    mut commands: Commands,
    main_image_res: Res<MainLoadingStepBackgroundImage>,
    loading_screen_tooltip_image_res: Res<LoadingScreenTooltipImage>,
    current_state: Res<State<GameState>>,
) {
    info!("current state is {:?}", current_state.get());
    info!("Setting main loading step background image");
    // info!("Main loading step background image is loaded");
    commands.spawn((Camera2d::default(), MainLoadingStepMainEntity));
    // TODO: try to convert to bsn! and check how to free resources and components and so on
    spawn_scene_with_whole_loading_screen(
        &mut commands,
        main_image_res,
        loading_screen_tooltip_image_res,
    );
}

fn spawn_scene_with_whole_loading_screen(
    commands: &mut Commands,
    main_image_res: Res<MainLoadingStepBackgroundImage>,
    loading_screen_tooltip_image_res: Res<LoadingScreenTooltipImage>,
) {
    commands.spawn_scene_list(ui(main_image_res, loading_screen_tooltip_image_res));
}

fn ui(
    main_image_res: Res<MainLoadingStepBackgroundImage>,
    loading_screen_tooltip_image_res: Res<LoadingScreenTooltipImage>,
) -> impl SceneList {
    let main_image = main_image_res.image.clone();
    let loading_screen_tooltip_image = loading_screen_tooltip_image_res.image.clone();
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
            TextLayout {
                justify: Justify::Center,
            }
            Outline {
                color: Color::WHITE,
                width: Val::Px(1.0),
            }
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
            Outline {
                color: Color::BLACK,
                width: Val::Px(1.0),
            }
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
                TextLayout {
                    justify: Justify::Start,
                }
                Outline {
                    color: Color::WHITE,
                    width: Val::Px(1.0),
                }
            ]
        ],
    }
}
