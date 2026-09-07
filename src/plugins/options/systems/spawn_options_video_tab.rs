use crate::plugins::options::components::options_video_tab_entity::OptionsVideoTabEntity;
use bevy::prelude::*;

pub fn spawn_options_video_tab(mut commands: Commands) {
    info!("Spawning options video tab");
    commands.spawn_scene_list(OptionsVideoTabEntity::as_scene_list());
}
