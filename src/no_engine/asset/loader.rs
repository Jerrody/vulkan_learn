use std::sync::Arc;

use glam::Vec3;

use crate::no_engine::{
    allocator::Allocator,
    objects::mesh::{Mesh, Vertex},
};

pub struct ObjectsLoader;

impl ObjectsLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load_obj_mesh(&self, path: &str, next_id: usize) -> Mesh {
        let load_options = tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        };

        let (models, _) = tobj::load_obj(path, &load_options).unwrap();
        let model = models.first().unwrap();
        let mesh = &model.mesh;

        let vertices = mesh
            .positions
            .chunks_exact(3)
            .zip(mesh.normals.chunks_exact(3))
            .map(|(position, normal)| {
                let position = Vec3::from_slice(position);

                Vertex {
                    position: position,
                    normal: Vec3::from_slice(normal),
                    color: position,
                }
            })
            .collect::<Vec<_>>();

        Mesh::new(vertices, next_id)
    }
}
