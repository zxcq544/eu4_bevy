use bevy::ecs::system::Commands;

use crate::InitialBootAtLeastTimeout;

pub fn setup_timeout(mut commands: Commands) {
    commands.insert_resource(InitialBootAtLeastTimeout::default());
}
