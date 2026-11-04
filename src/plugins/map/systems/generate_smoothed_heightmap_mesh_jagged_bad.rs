use crate::plugins::map::{
    components::{
        map_root_entity::MapRootEntity, terrain_chunk::TerrainChunk, water_plane_entity::WaterPlane,
    },
    resources::map_textures::MapTextures,
};
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use std::collections::HashMap;

// EU4 default values maps use 5632x2048 textures.
// We break this into manageable chunks (e.g., 64x64 or 128x128 vertices).
const CHUNK_SIZE: u32 = 64;
const SEA_LEVEL_THRESHOLD: f32 = 94.0 / 255.0; // EU4 water cutoff
const HEIGHT_SCALE: f32 = 25.0; // Visual extrusion modifier

// #[derive(Component)]
// pub struct TerrainChunk {
//     pub chunk_x: u32,
//     pub chunk_y: u32,
// }

/// Generates a single terrain chunk mesh using EU4 image data logic
fn generate_eu4_chunk_mesh_jagged_bad(
    heightmap: &Image,
    chunk_x: u32,
    chunk_y: u32,
) -> Option<Mesh> {
    let img_width = heightmap.texture_descriptor.size.width;
    let img_height = heightmap.texture_descriptor.size.height;

    let start_x = chunk_x * (CHUNK_SIZE - 1);
    let start_y = chunk_y * (CHUNK_SIZE - 1);

    if start_x >= img_width || start_y >= img_height {
        return None;
    }

    let end_x = (start_x + CHUNK_SIZE + 1).min(img_width);
    let end_y = (start_y + CHUNK_SIZE + 1).min(img_height);

    let actual_width = end_x - start_x;
    let actual_height = end_y - start_y;

    // Нам меньше 2х2 ячеек обрабатывать нет смысла
    if actual_width < 2 || actual_height < 2 {
        return None;
    }

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    // Кэш вершин, чтобы не дублировать точки на общих ребрах ячеек
    let mut vertex_cache: HashMap<String, u32> = HashMap::new();

    let get_height = |lx: u32, ly: u32| -> f32 {
        get_height_at_pixel(heightmap, start_x + lx, start_y + ly, img_width)
    };

    // Итерируемся по ячейкам 2х2 внутри чанка
    for y in 0..(actual_height - 1) {
        for x in 0..(actual_width - 1) {
            let global_x = start_x + x;
            let global_y = start_y + y;

            // Выборка 4-х углов текущей ячейки
            let h00 = get_height(x, y);
            let h10 = get_height(x + 1, y);
            let h11 = get_height(x + 1, y + 1);
            let h01 = get_height(x, y + 1);

            // Флаги: является ли вершина сушей
            let b00 = h00 > SEA_LEVEL_THRESHOLD;
            let b10 = h10 > SEA_LEVEL_THRESHOLD;
            let b11 = h11 > SEA_LEVEL_THRESHOLD;
            let b01 = h01 > SEA_LEVEL_THRESHOLD;

            // Вычисляем маску случая Marching Squares (0..15)
            let mut cell_case = 0;
            if b00 {
                cell_case |= 1;
            }
            if b10 {
                cell_case |= 2;
            }
            if b11 {
                cell_case |= 4;
            }
            if b01 {
                cell_case |= 8;
            }

            // Если вся ячейка под водой, полностью ее игнорируем (ваша оптимизация VRAM)
            if cell_case == 0 {
                continue;
            }

            // Субпиксельная интерполяция на ребрах ячейки
            let lerp_top = || {
                let t = (SEA_LEVEL_THRESHOLD - h00) / (h10 - h00);
                (x as f32 + t, SEA_LEVEL_THRESHOLD * HEIGHT_SCALE, y as f32)
            };
            let lerp_right = || {
                let t = (SEA_LEVEL_THRESHOLD - h10) / (h11 - h10);
                (
                    (x + 1) as f32,
                    SEA_LEVEL_THRESHOLD * HEIGHT_SCALE,
                    y as f32 + t,
                )
            };
            let lerp_bottom = || {
                let t = (SEA_LEVEL_THRESHOLD - h01) / (h11 - h01);
                (
                    x as f32 + t,
                    SEA_LEVEL_THRESHOLD * HEIGHT_SCALE,
                    (y + 1) as f32,
                )
            };
            let lerp_left = || {
                let t = (SEA_LEVEL_THRESHOLD - h00) / (h01 - h00);
                (x as f32, SEA_LEVEL_THRESHOLD * HEIGHT_SCALE, y as f32 + t)
            };

            // Позиции угловых вершин в локальных координатах чанка
            let c00 = (x as f32, h00 * HEIGHT_SCALE, y as f32);
            let c10 = ((x + 1) as f32, h10 * HEIGHT_SCALE, y as f32);
            let c11 = ((x + 1) as f32, h11 * HEIGHT_SCALE, (y + 1) as f32);
            let c01 = (x as f32, h01 * HEIGHT_SCALE, (y + 1) as f32);

            // Хелпер для добавления треугольника с правильным CCW-направлением для Bevy
            let mut add_tri = |p1, p2, p3| {
                add_triangle_to_buffers(
                    p1,
                    p2,
                    p3,
                    start_x,
                    start_y,
                    img_width,
                    img_height,
                    &mut positions,
                    &mut indices,
                    &mut uvs,
                    &mut vertex_cache,
                );
            };

            // Построение полигонов в зависимости от конфигурации ячейки
            match cell_case {
                15 => {
                    // Полностью суша (два стандартных треугольника)
                    add_tri(c00, c11, c10);
                    add_tri(c00, c01, c11);
                }
                1 => add_tri(c00, lerp_left(), lerp_top()),
                2 => add_tri(c10, lerp_top(), lerp_right()),
                3 => {
                    add_tri(c00, lerp_right(), c10);
                    add_tri(c00, lerp_left(), lerp_right());
                }
                4 => add_tri(c11, lerp_right(), lerp_bottom()),
                5 => {
                    add_tri(c00, lerp_left(), lerp_top());
                    add_tri(c11, lerp_right(), lerp_bottom());
                }
                6 => {
                    add_tri(c10, lerp_bottom(), c11);
                    add_tri(c10, lerp_top(), lerp_bottom());
                }
                7 => {
                    add_tri(c00, c11, c10);
                    add_tri(c00, lerp_bottom(), c11);
                    add_tri(c00, lerp_left(), lerp_bottom());
                }
                8 => add_tri(c01, lerp_bottom(), lerp_left()),
                9 => {
                    add_tri(c00, lerp_bottom(), lerp_top());
                    add_tri(c00, c01, lerp_bottom());
                }
                10 => {
                    add_tri(c10, lerp_top(), lerp_right());
                    add_tri(c01, lerp_bottom(), lerp_left());
                }
                11 => {
                    add_tri(c00, lerp_right(), c10);
                    add_tri(c00, lerp_bottom(), lerp_right());
                    add_tri(c00, c01, lerp_bottom());
                }
                12 => {
                    add_tri(c01, lerp_right(), c11);
                    add_tri(c01, lerp_left(), lerp_right());
                }
                13 => {
                    add_tri(c00, lerp_right(), lerp_top());
                    add_tri(c00, c11, lerp_right());
                    add_tri(c00, c01, c11);
                }
                14 => {
                    add_tri(c10, c01, c11);
                    add_tri(c10, lerp_left(), c01);
                    add_tri(c10, lerp_top(), lerp_left());
                }
                _ => {}
            }
        }
    }

    if positions.is_empty() {
        return None;
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    // 1. Сначала генерируем нормали меша (они нужны Bevy как база для расчета касательных)
    mesh.duplicate_vertices();
    mesh.compute_flat_normals(); // Использует TriangleList для генерации базовых нормалей фейсов

    // 2. КРИТИЧЕСКИЙ ШАГ ДЛЯ NORMAL MAP: Вычисляем касательные (Tangents) на основе позиций, UV и нормалей
    if let Err(err) = mesh.generate_tangents() {
        error!("Не удалось сгенерировать касательные для чанка: {:?}", err);
    }

    Some(mesh)
}

/// Вспомогательная функция сборки буферов геометрии
fn add_triangle_to_buffers(
    p1: (f32, f32, f32),
    p2: (f32, f32, f32),
    p3: (f32, f32, f32),
    start_x: u32,
    start_y: u32,
    img_width: u32,
    img_height: u32,
    positions: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    uvs: &mut Vec<[f32; 2]>,
    cache: &mut HashMap<String, u32>,
) {
    let mut register_vertex = |p: (f32, f32, f32)| -> u32 {
        let key = format!("{:.3}_{:.3}_{:.3}", p.0, p.1, p.2);
        if let Some(&idx) = cache.get(&key) {
            idx
        } else {
            let idx = positions.len() as u32;
            positions.push([p.0, p.1, p.2]);

            // Расчет глобальных UV-координат на основе положения чанка на карте
            let global_x = start_x as f32 + p.0;
            let global_z = start_y as f32 + p.2;
            uvs.push([global_x / img_width as f32, global_z / img_height as f32]);

            cache.insert(key, idx);
            idx
        }
    };

    indices.push(register_vertex(p1));
    indices.push(register_vertex(p2));
    indices.push(register_vertex(p3));
}

fn get_height_at_pixel(image: &Image, x: u32, y: u32, width: u32) -> f32 {
    let pixel_index = ((y * width + x) * 4) as usize;
    if let Some(ref data) = image.data {
        if pixel_index + 3 < data.len() {
            data[pixel_index] as f32 / 255.0
        } else {
            0.0
        }
    } else {
        0.0
    }
}

pub fn setup_terrain_system_jagged_bad(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    images: Res<Assets<Image>>,
    map_textures: Res<MapTextures>,
) {
    // 1. Load the flat maps exactly like EU4 does
    let colormap_summer_texture = map_textures.colormap_summer_texture.clone();
    let normal_map_texture = map_textures.normal_map_texture.clone();
    let heightmap_handle = map_textures.heightmap_texture.clone();

    if let Some(heightmap_img) = images.get(&heightmap_handle) {
        let img_width = heightmap_img.texture_descriptor.size.width;
        let img_height = heightmap_img.texture_descriptor.size.height;
        let center_x = img_width as f32 / 2.0;
        let center_z = img_height as f32 / 2.0;
        let chunks_x = (img_width + CHUNK_SIZE - 1) / CHUNK_SIZE;
        let chunks_y = (img_height + CHUNK_SIZE - 1) / CHUNK_SIZE;

        // 2. Loop across the whole world coordinate map
        commands
            .spawn((
                MapRootEntity,
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
            ))
            .with_children(|parent| {
                for cy in 0..chunks_y {
                    for cx in 0..chunks_x {
                        if let Some(chunk_mesh) =
                            generate_eu4_chunk_mesh_jagged_bad(heightmap_img, cx, cy)
                        {
                            let mesh_handle = meshes.add(chunk_mesh);
                            let material_handle = materials.add(StandardMaterial {
                                base_color: Color::from(bevy::color::palettes::css::WHITE),
                                // cull_mode: None, // uncomment if terrain is invisible
                                // Link Paradox textures here if desired (e.g. colormap.bmp / terrain.bmp)
                                base_color_texture: Some(colormap_summer_texture.clone()),
                                normal_map_texture: Some(normal_map_texture.clone()),
                                // Uncomment when neeed better looking terrain not metallic
                                // perceptual_roughness: 0.8,
                                // metallic: 0.0,
                                ..default()
                            });
                            let local_x = (cx * (CHUNK_SIZE - 1)) as f32 - center_x;
                            let local_z = (cy * (CHUNK_SIZE - 1)) as f32 - center_z;

                            // 3. Spawn utilizing Bevy 0.19 direct rendering components
                            parent.spawn((
                                Mesh3d(mesh_handle),
                                MeshMaterial3d(material_handle),
                                Transform::from_xyz(local_x, 0.0, local_z),
                                TerrainChunk {
                                    chunk_x: cx,
                                    chunk_y: cy,
                                },
                            ));
                        }
                    }
                }
            });
    }

    // 4. Render the Water Plane Layer below the land
    // A single simple plane fills all gaps skipped by the generator algorithm
    let water_material_handle = materials.add(StandardMaterial {
        base_color: Color::from(bevy::color::palettes::css::DARK_BLUE),
        perceptual_roughness: 0.15, // Низкая шероховатость делает воду глянцевой зеркальной
        metallic: 0.2,              // Небольшой металлический блеск для красивых отражений солнца
        reflectance: 0.5,           // Сила отражения света от поверхности воды
        ..default()
    });
    commands.spawn((
        WaterPlane,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
        MeshMaterial3d(water_material_handle),
        Transform::from_xyz(0.0, SEA_LEVEL_THRESHOLD * HEIGHT_SCALE, 0.0), // Snap strictly to EU4 sea level
    ));
}
