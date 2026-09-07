use crate::plugins::options::{
    components::options_background_entity::OptionsBackgroundEntity, resources::options_images::OptionsImages,
};
use bevy::prelude::*;

pub fn spawn_options_block(mut commands: Commands, options_images: Res<OptionsImages>) {
    info!("Spawning options block");
    commands.spawn_scene_list(OptionsBackgroundEntity::as_scene_list(&options_images));
}
