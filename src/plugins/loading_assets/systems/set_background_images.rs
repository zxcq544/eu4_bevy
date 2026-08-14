use bevy::prelude::*;

use crate::{
    core::states::GameState,
    plugins::loading_assets::{
        loading_assets::{MainLoadingStepBackgroundImage, MainLoadingStepMainEntity},
        resources::{
            loading_screen_status_image::LoadingScreenStatusImage,
            loading_screen_tooltip_image::LoadingScreenTooltipImage,
        },
    },
};
pub fn set_background_images(
    commands: Commands,
    // asset_server: Res<AssetServer>,
    main_image: Res<MainLoadingStepBackgroundImage>,
    loading_screen_status_image: Res<LoadingScreenStatusImage>,
    loading_screen_tooltip_image: Res<LoadingScreenTooltipImage>,
    current_state: Res<State<GameState>>,
) {
    set_main_loading_step_background_image(
        commands,
        main_image,
        loading_screen_status_image,
        loading_screen_tooltip_image,
        current_state,
    );
}

fn set_main_loading_step_background_image(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    main_image: Res<MainLoadingStepBackgroundImage>,
    loading_screen_status_image: Res<LoadingScreenStatusImage>,
    loading_screen_tooltip_image: Res<LoadingScreenTooltipImage>,
    current_state: Res<State<GameState>>,
) {
    info!("current state is {:?}", current_state.get());
    info!("Setting main loading step background image");
    // info!("Main loading step background image is loaded");
    commands.spawn((Camera2d::default(), MainLoadingStepMainEntity));
    // TODO: try to convert to bsn! and check how to free resources and components and so on
    commands
        .spawn((
            MainLoadingStepMainEntity,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::hidden(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    ..default()
                },
                ImageNode {
                    image: main_image.image.clone(),
                    ..default()
                },
            ));
        });
    commands
        .spawn((
            MainLoadingStepMainEntity,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                // bottom: Val::Vh(100.0),
                overflow: Overflow::hidden(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Vw(70.0),
                    height: Val::Vh(15.0),
                    // top: Val::Vh(-1.0),
                    // justify_content: JustifyContent::Center,
                    // align_items: AlignItems::Start,
                    ..default()
                },
                ImageNode {
                    image: loading_screen_status_image.image.clone(),
                    ..default()
                },
            ));
        });
    commands
        .spawn((
            MainLoadingStepMainEntity,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                // bottom: Val::Vh(100.0),
                overflow: Overflow::hidden(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Vw(70.0),
                    height: Val::Vh(15.0),

                    ..default()
                },
                ImageNode {
                    image: loading_screen_tooltip_image.image.clone(),
                    ..default()
                },
            ));
        });
}
