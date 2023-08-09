use ash::vk;

pub struct Buffer {
    buffer: vk::Buffer,
    allocation: vk_mem_alloc::Allocation,
}


