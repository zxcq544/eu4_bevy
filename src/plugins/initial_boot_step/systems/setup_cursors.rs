use crate::plugins::initial_boot_step::resources::cursor_handles::CursorHandles;
use bevy::prelude::*;
use bevy::window::{CursorIcon, CustomCursor, CustomCursorImage, PrimaryWindow};

pub fn setup_cursors(
    commands: &mut Commands,
    window: Single<Entity, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    cursors: Res<CursorHandles>,
) {
    // info!("current state is {:?}", current_state.get());
    // info!("initial boot step");
    // TODO: replace with normal test for loaded through Update
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
    }
}
