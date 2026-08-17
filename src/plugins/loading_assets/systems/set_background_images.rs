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
    main_image: Res<MainLoadingStepBackgroundImage>,
    loading_screen_tooltip_image: Res<LoadingScreenTooltipImage>,
    current_state: Res<State<GameState>>,
) {
    info!("current state is {:?}", current_state.get());
    info!("Setting main loading step background image");
    // info!("Main loading step background image is loaded");
    commands.spawn((Camera2d::default(), MainLoadingStepMainEntity));
    // TODO: try to convert to bsn! and check how to free resources and components and so on
    // Main background image
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
    // Text on top of screen with current loading info
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
                Text {
                    0: "Loading".to_string(),
                    ..default()
                },
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
            ));
        });
    // Image for tooltip
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
            parent
                .spawn((
                    Node {
                        width: Val::Vw(60.0),
                        height: Val::Vh(15.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ImageNode {
                        image: loading_screen_tooltip_image.image.clone(),
                        image_mode: NodeImageMode::Stretch,
                        ..default()
                    },
                    Outline {
                        color: Color::BLACK,
                        width: Val::Px(1.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            width: Val::Percent(75.0),
                            height: Val::Percent(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            overflow: Overflow::hidden(),
                            ..default()
                        },
                        Text {
                            0: "Tooltip".to_string(),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Start,
                            ..default()
                        },
                        Outline {
                            color: Color::WHITE,
                            width: Val::Px(1.0),
                            ..default()
                        },
                    ));
                });
        });
}
