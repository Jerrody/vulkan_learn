use crate::no_engine::{objects::mesh::MeshMetadata, Id};

use super::buffer::AllocatedBuffer;

pub struct AllocatedMesh {
    pub id: Id,
    pub metadata: MeshMetadata,
    pub vertex_buffer: AllocatedBuffer,
    pub index_buffer: AllocatedBuffer,
}

impl AllocatedMesh {
    #[inline(always)]
    pub fn new(
        id: Id,
        metadata: MeshMetadata,
        vertex_buffer: AllocatedBuffer,
        index_buffer: AllocatedBuffer,
    ) -> Self {
        Self {
            id,
            metadata,
            vertex_buffer,
            index_buffer,
        }
    }
}
