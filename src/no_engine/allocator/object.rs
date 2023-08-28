use ash::vk;

pub enum AllocatedObject {
    Buffer {
        buffer: vk::Buffer,
        allocation: vk_mem_alloc::Allocation,
    },
    Image {
        buffer: vk::Buffer,
        allocation: vk_mem_alloc::Allocation,
    }
}

pub struct Allocated {
    pub id: usize,
    pub offset: u64,
    pub size: u64,
    pub object_type: super::ObjectType,
    pub object: AllocatedObject,
}

impl Allocated {
    pub fn new_buffer(
        id: usize,
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
            object: AllocatedObject::Buffer { buffer, allocation },
        }
    }

    pub fn new_image(
        id: usize,
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
            object: AllocatedObject::Image { buffer, allocation },
        }
    }
}
