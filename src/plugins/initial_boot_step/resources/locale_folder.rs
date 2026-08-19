use bevy::prelude::*;
use bevy::{asset::LoadedFolder, ecs::resource::Resource};

#[derive(Resource)]
pub struct LocaleFolder {
    pub folder: Handle<LoadedFolder>,
}
