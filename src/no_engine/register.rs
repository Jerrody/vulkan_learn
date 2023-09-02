use ash::vk;

use super::allocator::mesh::AllocatedMesh;

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
    pub fn register_mesh(&mut self, allocated_mesh: AllocatedMesh) {
        self.allocated_meshes.push(allocated_mesh);

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
            .map(|mesh| mesh.vertex_buffer.buffer)
            .collect();

        self.offsets = self
            .get_meshes()
            .iter()
            .map(|mesh| mesh.vertex_buffer.offset)
            .collect();
    }
}
