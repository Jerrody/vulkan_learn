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
    pub indices_count: u32,
}

impl MeshMetadata {
    pub fn new(id: Id, vertices_count: u32, indices_count: u32) -> Self {
        Self {
            id,
            vertices_count,
            indices_count,
        }
    }
}

pub struct Mesh {
    pub metadata: MeshMetadata,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub is_uploaded: bool,
}

impl Mesh {
    pub fn new(id: Id, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let metadata = MeshMetadata::new(id, vertices.len() as u32, indices.len() as u32);
        Self {
            metadata,
            vertices,
            indices,
            is_uploaded: false,
        }
    }
}
