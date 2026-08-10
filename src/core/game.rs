use bevy::prelude::*;

// use crate::plugins::{
//     ai::AiPlugin, buildings::BuildingsPlugin, camera::CameraPlugin, economy::EconomyPlugin,
//     fog_of_war::FogOfWarPlugin, input::InputPlugin, map::MapPlugin, pathfinding::PathfindingPlugin,
//     ui::UiPlugin, units::UnitsPlugin,
// };
use crate::InitialBootStepPlugin;
use crate::LoadingAssetsPlugin;
use crate::core::states::GameState;
use crate::plugins::MapPlugin;
use crate::plugins::MonitorChooserPlugin;
use crate::plugins::MusicPlayerPlugin;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // 1. Register core state machine
        app.init_state::<GameState>();
        app.add_systems(Startup, global_setup);

        // 2. Add subsystem plugins. Each plugin is responsible
        //    for registering its own components/resources/events
        //    and gating systems on GameState::Playing.
        app.add_plugins((
            MonitorChooserPlugin,
            InitialBootStepPlugin,
            LoadingAssetsPlugin,
            //     CameraPlugin,
            //     InputPlugin,
            MusicPlayerPlugin,
            MapPlugin,
            //     PathfindingPlugin,
            //     UnitsPlugin,
            //     BuildingsPlugin,
            //     EconomyPlugin,
            //     FogOfWarPlugin,
            //     AiPlugin,
            // UiPlugin,
        ));

        // 3. Configure system sets if cross-plugin ordering matters
        // app.configure_sets(
        //     Update,
        //     (CoreSet::Input, CoreSet::Simulation, CoreSet::Presentation).chain(),
        // );

        // 4. Global setup that doesn’t belong to any subsystem
    }
}

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub enum CoreSet {
//     Input,
//     Simulation,
//     Presentation,
// }

fn global_setup(mut commands: Commands) {
    // Spawn a global camera rig, ambient light, debug grid, etc.
    commands.spawn((
        Camera3d::default(),
        // crate::plugins::camera::components::RtsCamera::default(),
    ));
}
