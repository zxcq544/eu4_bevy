use crate::plugins::main_menu::{
    components::{
        continue_game_entity::ContinueGameEntity, main_menu_entity::MainMenuEntity,
        rotating_cube::RotatingCube,
    },
    resources::main_menu_all_images::MainMenuAllImages,
};
use bevy::prelude::*;
use bevy_fluent::Localization;
use fonts::FontHandles;

pub fn spawn_main_menu_scene_with_cam(
    main_menu_all_images_res: Res<MainMenuAllImages>,
    mut commands: Commands,
    localization_res: Res<Localization>,
    fonts: Res<FontHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let show_continue_button = true;
    info!("Setting up main menu background");
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainMenuEntity,
    ));
    commands.spawn_scene_list(MainMenuEntity::as_scene_list(
        &main_menu_all_images_res,
        &localization_res,
        &fonts,
    ));
    if show_continue_button {
        commands.spawn_scene_list(ContinueGameEntity::as_scene_list(
            &main_menu_all_images_res,
            &localization_res,
            &fonts,
        ));
    }
    // spawn cube in center
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            // Если не хотите добавлять свет, сделайте материал светящимся:
            // unlit: true,
            ..default()
        })),
        Transform::from_translation(Vec3::ZERO),
        RotatingCube,
    ));
    // 3. Добавляем источник света (иначе куб будет черным)
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
