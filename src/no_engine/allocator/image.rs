use ash::vk;

use crate::no_engine::Id;

pub struct AllocatedImage {
    pub id: Id,
    pub format: vk::Format,
    pub image: vk::Image,
    pub allocation: vk_mem_alloc::Allocation,
}

impl AllocatedImage {
    pub fn new(
        id: Id,
        format: vk::Format,
        image: vk::Image,
        allocation: vk_mem_alloc::Allocation,
    ) -> Self {
        Self {
            id,
            format,
            image,
            allocation,
        }
    }
}
