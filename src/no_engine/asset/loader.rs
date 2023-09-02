use glam::Vec3;

use crate::no_engine::{
    id::Id,
    objects::mesh::{Mesh, Vertex},
};

pub struct ObjectsLoader;

impl ObjectsLoader {
    const VERTICIES_PER_TRIANGLE: usize = 3;

    pub fn new() -> Self {
        Self
    }

    pub fn load_obj_mesh(&self, path: std::path::PathBuf, id: Id) -> Option<Mesh> {
        let load_options = tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        };

        if let Ok((models, _)) = tobj::load_obj(path, &load_options) {
            if let Some(model) = models.first() {
                let mesh = &model.mesh;

                let vertices = mesh
                    .positions
                    .chunks(Self::VERTICIES_PER_TRIANGLE)
                    .zip(mesh.normals.chunks(3))
                    .map(|(position, normal)| {
                        let position = Vec3::from_slice(position);
                        let normal = Vec3::from_slice(normal);
                        Vertex {
                            position,
                            normal,
                            color: normal,
                        }
                    })
                    .collect();

                return Some(Mesh::new(id, vertices, mesh.indices.clone()));
            }
        }

        None
    }
}
