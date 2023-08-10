use glam::Vec3;

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
}

pub struct Mesh {
    pub id: usize,
    pub vertices: Vec<Vertex>,
    pub is_loaded: bool,
}

impl Mesh {
    pub fn new(verticies: Vec<Vertex>, id: usize) -> Self {
        Self {
            vertices: verticies,
            id,
            is_loaded: Default::default(),
        }
    }
}
