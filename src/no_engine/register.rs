use crate::no_engine::objects::mesh::MeshMetadata;

use super::allocator::buffer::AllocatedBuffer;

pub struct AllocatedMesh {
    mesh_metadata: MeshMetadata,
    allocated_buffer: AllocatedBuffer,
}

impl AllocatedMesh {
    pub fn new(mesh_metadata: MeshMetadata, allocated_buffer: AllocatedBuffer) -> Self {
        Self {
            mesh_metadata,
            allocated_buffer,
        }
    }
}

// TODO: Maybe move to the future scene manager?
#[derive(Default)]
pub struct Register {
    allocated_meshes: Vec<AllocatedMesh>,
}

impl Register {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline(always)]
    pub fn register_mesh(
        &mut self,
        mesh_metadata: MeshMetadata,
        allocated_buffer: AllocatedBuffer,
    ) {
        self.allocated_meshes
            .push(AllocatedMesh::new(mesh_metadata, allocated_buffer));
    }

    #[inline(always)]
    pub fn get_meshes(&self) -> &[AllocatedMesh] {
        &self.allocated_meshes
    }
}
