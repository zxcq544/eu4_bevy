use bevy::prelude::*;

pub fn move_cam_and_insert_cube(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // mut main_camera: Query<&mut Transform, With<MainCameraEntity>>,
) {
    // Main camera
    // if let Ok(mut transform) = main_camera.single_mut() {
    //     *transform = Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    // }
    // spawn cube in center. Main 3D Map will be somewhere here
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1000.0)))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::WHITE,
    //         // unlit: true,
    //         ..default()
    //     })),
    //     Transform::from_translation(Vec3 {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 1000.0,
    //     }), // Позиция куба в центре карты
    //     RotatingCube,
    // ));
    // light
    // commands.spawn((
    //     PointLight {
    //         shadow_maps_enabled: true,
    //         ..default()
    //     },
    //     Transform::from_xyz(4.0, 8.0, 4.0),
    // ));
    // TODO: uncomment when lights are implemented and actual mesh position available
    commands.spawn((
        DirectionalLight {
            illuminance: 4000.0,
            shadow_maps_enabled: true,
            contact_shadows_enabled: true,
            // shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1000.0, 2000.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
