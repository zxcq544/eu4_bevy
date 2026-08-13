use crate::plugins::initial_boot_step::resources::initial_boot_at_least_timer::InitialBootAtLeastTimeout;
use bevy::ecs::system::Commands;

pub fn setup_timeout(mut commands: Commands) {
    commands.insert_resource(InitialBootAtLeastTimeout::default());
}
