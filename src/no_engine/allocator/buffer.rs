use ash::vk;

use crate::no_engine::Id;

pub struct AllocatedBuffer {
    pub id: Id,
    pub offset: u64,
    pub size: u64,
    pub object_type: super::ObjectType,
    pub buffer: vk::Buffer,
    pub allocation: vk_mem_alloc::Allocation,
}

impl AllocatedBuffer {
    pub fn new(
        id: Id,
        size: u64,
        object_type: super::ObjectType,
        buffer: vk::Buffer,
        allocation: vk_mem_alloc::Allocation,
    ) -> Self {
        Self {
            id,
            offset: Default::default(),
            size,
            object_type,
            buffer,
            allocation,
        }
    }
}
