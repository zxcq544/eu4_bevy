use bevy::prelude::*;

use crate::{core::states::GameState, plugins::main_menu::components::rotating_cube::RotatingCube};

#[derive(Component)]
pub struct MainCameraEntity;

pub fn insert_cam_and_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Main camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCameraEntity,
    ));
    // spawn cube in center. Main 3D Map will be somewhere here
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            // unlit: true,
            ..default()
        })),
        Transform::from_translation(Vec3::ZERO),
        RotatingCube,
    ));
    // light
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    next_state.set(GameState::MainMenu);
}
