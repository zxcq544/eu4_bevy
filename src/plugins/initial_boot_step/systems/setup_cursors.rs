use bevy::prelude::*;
use bevy::window::{CursorIcon, CustomCursor, CustomCursorImage, PrimaryWindow};

use crate::{CursorHandles, states::GameState};

pub fn setup_cursors(
    mut commands: Commands,
    window: Single<Entity, With<PrimaryWindow>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    cursors: Res<CursorHandles>,
) {
    info!("current state is {:?}", current_state.get());
    info!("initial boot step");

    if asset_server.is_loaded_with_dependencies(&cursors.normal) {
        commands
            .entity(*window)
            .insert(CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
                handle: cursors.normal.clone(),
                hotspot: (0, 0),
                texture_atlas: None,
                flip_x: false,
                flip_y: false,
                rect: None,
                ..default()
            })));
        next_state.set(GameState::LoadingAssets);
    }
}
