use bevy::{
    prelude::*,
    window::{CursorIcon, CustomCursor, CustomCursorImage},
};

use crate::states::GameState;
pub struct InitialBootStepPlugin;
/// This plugin is responsible for setting up the initial boot step
/// This one sets default cursor and initial background and camera
impl Plugin for InitialBootStepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initial_boot_step);
    }
}

pub fn initial_boot_step(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    info!("current state is {:?}", current_state.get());
    info!("initial boot step");
    let cursor_handle: Handle<Image> = asset_server.load("gfx/cursors/normal.png");

    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: cursor_handle,
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
