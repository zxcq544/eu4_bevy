use bevy::prelude::*;

use crate::core::states::GameState;

pub struct LoadingAssetsPlugin;

impl Plugin for LoadingAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            loading_assets.run_if(in_state(GameState::LoadingAssets)),
        );
    }
}

pub fn loading_assets(
    // mut commands: Commands,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("current state is {:?}", current_state.get());
    info!("loading assets");
    next_state.set(GameState::MainMenu);
}
