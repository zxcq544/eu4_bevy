use crate::plugins::map::resources::map_textures::MapTextures;
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

pub fn load_map_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let heightmap_texture = asset_server.load("map/heightmap.bmp");
    commands.insert_resource(MapTextures { heightmap_texture });
}
