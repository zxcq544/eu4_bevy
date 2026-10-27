use bevy::prelude::*;
#[derive(Resource)]
pub struct MapTextures {
    pub colormap_summer_texture: Handle<Image>,
    pub heightmap_texture: Handle<Image>,
}
