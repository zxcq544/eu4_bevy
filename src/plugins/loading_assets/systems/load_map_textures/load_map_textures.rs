use crate::plugins::map::resources::map_textures::MapTextures;
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    image::ImageLoaderSettings,
};

pub fn load_map_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let colormap_summer_texture = asset_server.load("map/terrain/colormap_summer.dds");
    let normal_map_texture = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| {
            settings.is_srgb = false;
        })
        .load("map/world_normal.bmp");
    let heightmap_texture = asset_server.load("map/heightmap.bmp");
    commands.insert_resource(MapTextures {
        colormap_summer_texture,
        normal_map_texture,
        heightmap_texture,
    });
}
