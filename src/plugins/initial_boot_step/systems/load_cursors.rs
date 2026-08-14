use crate::plugins::initial_boot_step::resources::cursor_handles::CursorHandles;
use bevy::prelude::*;

pub fn load_cursors(mut commands: Commands, asset_server: Res<AssetServer>) {
    let normal = asset_server.load("gfx/cursors/normal.png");
    let build_cavalry = asset_server.load("gfx/cursors/build_cavalry.png");
    let cursors = CursorHandles {
        normal,
        build_cavalry,
    };
    commands.insert_resource(cursors);
}
