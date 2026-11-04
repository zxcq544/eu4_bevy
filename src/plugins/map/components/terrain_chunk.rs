use bevy::ecs::component::Component;

#[derive(Component)]
pub struct TerrainChunk {
    pub chunk_x: u32,
    pub chunk_y: u32,
}
