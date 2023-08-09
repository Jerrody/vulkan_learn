pub mod buffer;

use ash::vk;

pub struct Allocator {
    allocator: vk_mem_alloc::Allocator,
}

impl Allocator {
    pub fn new(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        device: &ash::Device,
    ) -> Self {
        let allocator = unsafe {
            vk_mem_alloc::create_allocator(instance, physical_device, device, None).unwrap()
        };

        Self { allocator }
    }
}
