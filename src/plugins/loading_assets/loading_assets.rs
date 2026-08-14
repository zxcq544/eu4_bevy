use crate::{
    core::states::GameState,
    plugins::loading_assets::systems::set_background_images::set_background_images,
};
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
        app.add_systems(OnEnter(GameState::LoadingAssets), set_background_images);
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
