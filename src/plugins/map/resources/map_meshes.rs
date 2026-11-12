use bevy::{asset::Handle, ecs::resource::Resource, mesh::Mesh};

#[derive(Resource)]
pub struct MapMeshes {
    pub map_full: Handle<Mesh>,
}
