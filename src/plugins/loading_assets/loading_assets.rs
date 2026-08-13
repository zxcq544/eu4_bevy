use crate::core::states::GameState;
use bevy::prelude::*;

pub struct LoadingAssetsPlugin;

#[derive(Resource)]
pub struct MainLoadingStepBackgroundImage {
    pub image: Handle<Image>,
}

#[derive(Component)]
pub struct MainLoadingStepMainEntity;

impl Plugin for LoadingAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::LoadingAssets),
            set_main_loading_step_background_image,
        );
        // app.add_systems(
        //     Update,
        //     loading_assets.run_if(in_state(GameState::LoadingAssets)),
        // );
    }
}

// pub fn loading_assets(
//     // mut commands: Commands,
//     current_state: Res<State<GameState>>,
//     mut next_state: ResMut<NextState<GameState>>,
// ) {
//     info!("current state is {:?}", current_state.get());
//     info!("loading assets");
//     next_state.set(GameState::MainMenu);
// }

pub fn set_main_loading_step_background_image(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    main_image: Res<MainLoadingStepBackgroundImage>,
    current_state: Res<State<GameState>>,
) {
    info!("current state is {:?}", current_state.get());
    info!("Setting main loading step background image");
    if asset_server.is_loaded(&main_image.image) {
        info!("Main loading step background image is loaded");
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
    }
}
