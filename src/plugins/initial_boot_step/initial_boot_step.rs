use bevy::prelude::*;

use crate::states::GameState;
pub struct InitialBootStepPlugin;
/// This plugin is responsible for setting up the initial boot step
/// Usually this could be used to calculate checksums and so on.
/// Currently just empty.
impl Plugin for InitialBootStepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initial_boot_step);
    }
}

pub fn initial_boot_step(
    mut commands: Commands,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("current state is {:?}", current_state.get());
    info!("initial boot step");
    next_state.set(GameState::LoadingAssets);
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}
