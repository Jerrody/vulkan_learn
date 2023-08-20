use glam::Vec3;

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
}

#[derive(Clone, Copy)]
pub struct MeshMetadata {
    pub id: usize,
    pub vertices_count: usize,
    pub indices_count: usize,
}

impl MeshMetadata {
    pub fn new(id: usize, vertices_count: usize, indices_count: usize) -> Self {
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
    pub fn new(verticies: Vec<Vertex>, id: usize) -> Self {
        Self {
            mesh_metadata: MeshMetadata::new(id, verticies.len(), Default::default()),
            vertices: verticies,
            is_uploaded: Default::default(),
        }
    }
}
