use crate::plugins::map::resources::map_textures::MapTextures;
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

// EU4 default values maps use 5632x2048 textures.
// We break this into manageable chunks (e.g., 64x64 or 128x128 vertices).
const CHUNK_SIZE: u32 = 64;
const SEA_LEVEL_THRESHOLD: f32 = 94.0 / 255.0; // EU4 water cutoff
const HEIGHT_SCALE: f32 = 25.0; // Visual extrusion modifier

#[derive(Component)]
pub struct WaterPlane;

#[derive(Component)]
pub struct TerrainChunk {
    pub chunk_x: u32,
    pub chunk_y: u32,
}

/// Generates a single terrain chunk mesh using EU4 image data logic
pub fn generate_eu4_chunk_mesh(heightmap: &Image, chunk_x: u32, chunk_y: u32) -> Option<Mesh> {
    let img_width = heightmap.texture_descriptor.size.width;
    let img_height = heightmap.texture_descriptor.size.height;

    // IMPORTANT: Shift the chunk start by the chunk overlap of the adjacent chunks
    let start_x = chunk_x * (CHUNK_SIZE - 1);
    let start_y = chunk_y * (CHUNK_SIZE - 1);

    if start_x >= img_width || start_y >= img_height {
        return None;
    }

    // CHUNK_SIZE + 1 so that the right/lower corner matches the next chunk
    let end_x = (start_x + CHUNK_SIZE + 1).min(img_width);
    let end_y = (start_y + CHUNK_SIZE + 1).min(img_height);

    let actual_width = end_x - start_x;
    let actual_height = end_y - start_y;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let mut vertex_index_map = vec![None; (actual_width * actual_height) as usize];
    let mut current_vertex_count = 0u32;

    for y in 0..actual_height {
        for x in 0..actual_width {
            let global_x = start_x + x;
            let global_y = start_y + y;

            let height_val = get_height_at_pixel(heightmap, global_x, global_y, img_width);

            // CRITICAL PARADOX ENGINE OPTIMIZATION: Skip allocating flat water vertices entirely to protect VRAM performance.
            if height_val <= SEA_LEVEL_THRESHOLD {
                continue;
            }

            let pos_x = x as f32;
            let pos_z = y as f32;
            let pos_y = height_val * HEIGHT_SCALE;

            // --- Calculating normal vector from heightmap ---

            let h_left =
                get_height_at_pixel(heightmap, global_x.saturating_sub(1), global_y, img_width);
            let h_right = get_height_at_pixel(
                heightmap,
                (global_x + 1).min(img_width - 1),
                global_y,
                img_width,
            );
            let h_down =
                get_height_at_pixel(heightmap, global_x, global_y.saturating_sub(1), img_width);
            let h_up = get_height_at_pixel(
                heightmap,
                global_x,
                (global_y + 1).min(img_height - 1),
                img_width,
            );

            // Calculate normal vector
            let vec_x = Vec3::new(2.0, (h_right - h_left) * HEIGHT_SCALE, 0.0);
            let vec_z = Vec3::new(0.0, (h_up - h_down) * HEIGHT_SCALE, 2.0);
            let normal = vec_z.cross(vec_x).normalize();

            positions.push([pos_x, pos_y, pos_z]);
            normals.push([normal.x, normal.y, normal.z]);
            uvs.push([
                global_x as f32 / img_width as f32,
                global_y as f32 / img_height as f32,
            ]);

            let local_array_idx = (y * actual_width + x) as usize;
            vertex_index_map[local_array_idx] = Some(current_vertex_count);
            current_vertex_count += 1;
        }
    }

    if positions.is_empty() {
        return None;
    }

    // Build Triangle Indices
    let mut indices: Vec<u32> = Vec::new();
    for y in 0..(actual_height - 1) {
        for x in 0..(actual_width - 1) {
            let tl_idx = (y * actual_width + x) as usize;
            let tr_idx = (y * actual_width + (x + 1)) as usize;
            let bl_idx = ((y + 1) * actual_width + x) as usize;
            let br_idx = ((y + 1) * actual_width + (x + 1)) as usize;

            if let (Some(tl), Some(tr), Some(bl), Some(br)) = (
                vertex_index_map[tl_idx],
                vertex_index_map[tr_idx],
                vertex_index_map[bl_idx],
                vertex_index_map[br_idx],
            ) {
                // Triangle 1
                // in case of wrong normals use this code
                // indices.push(tl);
                // indices.push(tr);
                // indices.push(br);
                indices.push(tl);
                indices.push(bl);
                indices.push(tr);

                // Triangle 2
                // in case of wrong normals use this code
                // indices.push(tr);
                // indices.push(br);
                // indices.push(bl);
                indices.push(tr);
                indices.push(bl);
                indices.push(br);
            }
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    Some(mesh)
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

pub fn setup_terrain_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    images: Res<Assets<Image>>,
    map_textures: Res<MapTextures>,
) {
    // 1. Load the flat maps exactly like EU4 does
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
                WaterPlane,
                // TODO: remove this and instead move camera to show this map properly
                Transform {
                    translation: Vec3::new(0.0, -100.0, center_z),
                    rotation: Quat::from_rotation_x(-90.0f32.to_radians())
                        * Quat::from_rotation_y(180.0f32.to_radians())
                        * Quat::from_rotation_z(180.0f32.to_radians()),
                    ..default()
                },
                Visibility::default(),
                InheritedVisibility::default(),
            ))
            .with_children(|parent| {
                for cy in 0..chunks_y {
                    for cx in 0..chunks_x {
                        if let Some(chunk_mesh) = generate_eu4_chunk_mesh(heightmap_img, cx, cy) {
                            let mesh_handle = meshes.add(chunk_mesh);
                            let material_handle = materials.add(StandardMaterial {
                                base_color: Color::from(bevy::color::palettes::css::GREEN),
                                // cull_mode: None,
                                // Link Paradox textures here if desired (e.g. colormap.bmp / terrain.bmp)
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
    // commands.spawn((
    //     WaterPlane,
    //     Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::from(bevy::color::palettes::css::BLUE),
    //         base_color_texture: Some(map_textures.heightmap_texture.clone()),
    //         // metallic: 0.1,
    //         ..default()
    //     })),
    //     // Transform::from_xyz(
    //     //     ((img_width / 2) as f32),
    //     //     SEA_LEVEL_THRESHOLD * HEIGHT_SCALE,
    //     //     ((img_height / 2) as f32),
    //     // ),
    //     // Transform::from_xyz(0.0, 94.0 / 255.0 * HEIGHT_SCALE, 0.0), // Snap strictly to EU4 sea level
    // ));
}
