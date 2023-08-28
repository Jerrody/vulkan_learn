use glam::Vec3;

use crate::no_engine::Id;

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
}

#[derive(Clone, Copy)]
pub struct MeshMetadata {
    pub id: Id,
    pub vertices_count: u32,
    pub indices_count: usize,
}

impl MeshMetadata {
    pub fn new(id: Id, vertices_count: u32, indices_count: usize) -> Self {
        Self {
            id,
            vertices_count,
            indices_count,
        }
    }
}

pub struct Mesh {
    pub mesh_metadata: MeshMetadata,
    pub vertices: Vec<Vertex>,
    pub is_uploaded: bool,
}

impl Mesh {
    pub fn new(verticies: Vec<Vertex>, id: Id) -> Self {
        Self {
            mesh_metadata: MeshMetadata::new(id, verticies.len() as u32, Default::default()),
            vertices: verticies,
            is_uploaded: Default::default(),
        }
    }
}
