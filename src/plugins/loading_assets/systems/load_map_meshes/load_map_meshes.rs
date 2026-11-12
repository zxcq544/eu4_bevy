use crate::plugins::map::resources::map_meshes::MapMeshes;
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    gltf::GltfAssetLabel,
};

pub fn load_map_meshes(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Use small mesh for now. Maybe switch to full size 20 mln triangle if needed later.
    let map_full = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("map/terrain/terrain_divided_by_4_width_size.glb"),
    );
    commands.insert_resource(MapMeshes { map_full });
}
