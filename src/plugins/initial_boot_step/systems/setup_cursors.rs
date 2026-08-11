use bevy::prelude::*;
use bevy::window::{CursorIcon, CustomCursor, CustomCursorImage, PrimaryWindow};

use crate::{CursorHandles, states::GameState};

pub fn setup_cursors(
    mut commands: Commands,
    window: Single<Entity, With<PrimaryWindow>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    info!("current state is {:?}", current_state.get());
    info!("initial boot step");
    let cursor_handles = load_cursors(asset_server);

    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: cursor_handles.normal,
            hotspot: (0, 0),
            texture_atlas: None,
            flip_x: false,
            flip_y: false,
            rect: None,
            ..default()
        })));
    next_state.set(GameState::LoadingAssets);
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}

fn load_cursors(asset_server: Res<AssetServer>) -> CursorHandles {
    let normal = asset_server.load("gfx/cursors/normal.png");
    let build_cavalry = asset_server.load("gfx/cursors/build_cavalry.png");
    CursorHandles {
        normal,
        build_cavalry,
    }
}
