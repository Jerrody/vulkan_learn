use ash::vk;

use crate::no_engine::objects::mesh::MeshMetadata;

use super::allocator::buffer::AllocatedBuffer;

pub struct AllocatedMesh {
    pub mesh_metadata: MeshMetadata,
    pub allocated_buffer: AllocatedBuffer,
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
    buffers: Vec<vk::Buffer>,
    offsets: Vec<u64>,
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

        self.accumulate_data();
    }

    #[inline(always)]
    pub fn get_meshes(&self) -> &[AllocatedMesh] {
        &self.allocated_meshes
    }

    #[inline(always)]
    pub fn get_buffers(&self) -> &[vk::Buffer] {
        &self.buffers
    }

    #[inline(always)]
    pub fn get_offsets(&self) -> &[u64] {
        &self.offsets
    }

    #[inline(always)]
    fn accumulate_data(&mut self) {
        self.buffers = self
            .get_meshes()
            .iter()
            .map(|mesh| mesh.allocated_buffer.buffer)
            .collect();

        self.offsets = self
            .get_meshes()
            .iter()
            .map(|mesh| mesh.allocated_buffer.offset)
            .collect();
    }
}
