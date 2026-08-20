use crate::plugins::initial_boot_step::resources::cursor_handles::CursorHandles;
use bevy::prelude::*;
use bevy::window::{CursorIcon, CustomCursor, CustomCursorImage, PrimaryWindow};

pub fn setup_cursors(
    mut commands: Commands,
    window: Single<Entity, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    cursors: Res<CursorHandles>,
    current_cursor_icon: Query<&CursorIcon, With<PrimaryWindow>>,
) {
    if asset_server.is_loaded_with_dependencies(&cursors.normal) {
        let mut is_custom_cursor = false;
        if let Ok(current_cursor) = current_cursor_icon.single() {
            match current_cursor {
                CursorIcon::Custom(_) => is_custom_cursor = true,
                _ => is_custom_cursor = false,
            }
        }
        if is_custom_cursor == false {
            info!("Setting cursor");
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
            info!("Cursor is set");
        }
    }
}
