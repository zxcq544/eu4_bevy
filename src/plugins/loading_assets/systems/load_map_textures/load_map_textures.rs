use crate::plugins::map::resources::map_textures::MapTextures;
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

pub fn load_map_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let colormap_summer_texture = asset_server.load("map/terrain/colormap_summer.dds");
    let heightmap_texture = asset_server.load("map/heightmap.bmp");
    commands.insert_resource(MapTextures {
        colormap_summer_texture,
        heightmap_texture,
    });
}
