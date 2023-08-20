use ash::vk;

pub struct AllocatedBuffer {
    pub id: usize,
    pub object_type: super::ObjectType,
    pub buffer: vk::Buffer,
    pub allocation: vk_mem_alloc::Allocation,
}

impl AllocatedBuffer {
    pub fn new(
        id: usize,
        object_type: super::ObjectType,
        buffer: vk::Buffer,
        allocation: vk_mem_alloc::Allocation,
    ) -> Self {
        Self {
            id,
            object_type,
            buffer,
            allocation,
        }
    }
}
