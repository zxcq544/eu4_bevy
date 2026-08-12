use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowPosition, WindowResolution};
use settings::Settings;

use crate::CursorHandles;
use crate::states::GameState;

pub fn setup_window_monitor(
    settings: Res<Settings>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let monitor_index = settings.monitor_index;
    let resolution_width = settings.resolution_width;
    let resolution_height = settings.resolution_height;
    info!("Monitor index from settings: {}", monitor_index);
    if let Ok(mut window) = window_query.single_mut() {
        // This explicitly moves and centers the window on Monitor 0 (or 1, etc.)
        window.resolution = WindowResolution::new(resolution_width, resolution_height)
            .with_scale_factor_override(1.0);
        window.position = WindowPosition::Centered(MonitorSelection::Index(monitor_index));
        window.visible = true;
    }
    // Have to start loading cursors here so we have beautiful cursor as soon as possible
    // which is on booting screen
    load_cursors(commands, asset_server);
    next_state.set(GameState::Boot);
}

fn load_cursors(mut commands: Commands, asset_server: Res<AssetServer>) {
    let normal = asset_server.load("gfx/cursors/normal.png");
    let build_cavalry = asset_server.load("gfx/cursors/build_cavalry.png");
    let cursors = CursorHandles {
        normal,
        build_cavalry,
    };
    commands.insert_resource(cursors);
}
