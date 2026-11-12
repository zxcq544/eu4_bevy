use crate::plugins::map::{
    components::{map_root_entity::MapRootEntity, water_plane_entity::WaterPlane},
    resources::map_meshes::MapMeshes,
};
use bevy::prelude::*;

pub fn spawn_map_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // images: Res<Assets<Image>>,
    map_meshes: Res<MapMeshes>,
    // map_textures: Res<MapTextures>,
) {
    // let colormap_summer_texture = map_textures.colormap_summer_texture.clone();
    // let normal_map_texture = map_textures.normal_map_texture.clone();
    // let heightmap_handle = map_textures.heightmap_texture.clone();
    let map_root_mesh = map_meshes.map_full.clone();

    // 2. Loop across the whole world coordinate map
    commands.spawn((
        MapRootEntity,
        Mesh3d(map_root_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::linear_rgba(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        })),
        // TODO: remove this and instead move camera to show this map properly
        Transform {
            // translation: Vec3::new(0.0, -100.0, center_z),
            // rotation: Quat::from_rotation_x(-90.0f32.to_radians())
            //     * Quat::from_rotation_y(180.0f32.to_radians())
            //     * Quat::from_rotation_z(180.0f32.to_radians()),
            ..default()
        },
        Visibility::default(),
        InheritedVisibility::default(),
    ));

    // 4. Render the Water Plane Layer below the land
    // A single simple plane fills all gaps skipped by the generator algorithm
    // let water_material_handle = materials.add(StandardMaterial {
    //     base_color: Color::linear_rgba(0.0, 0.0, 0.6, 0.8),
    //     alpha_mode: AlphaMode::Opaque, // Maybe use Multiply and 0.7 alpha for beauty
    //     perceptual_roughness: 0.15,    // Низкая шероховатость делает воду глянцевой зеркальной
    //     metallic: 0.2,    // Небольшой металлический блеск для красивых отражений солнца
    //     reflectance: 0.5, // Сила отражения света от поверхности воды
    //     ..default()
    // });
    // commands.spawn((
    //     WaterPlane,
    //     Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
    //     MeshMaterial3d(water_material_handle),
    //     Transform::from_xyz(0.0, 94.0 / 255.0, 0.0), // Snap strictly to EU4 sea level
    // ));
}
