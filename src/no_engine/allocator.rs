pub mod buffer;

use self::buffer::AllocatedBuffer;

use super::objects::ObjectType;
use ash::vk;

use super::objects::mesh::Mesh;

pub struct Allocator {
    allocator: vk_mem_alloc::Allocator,
    allocated_buffers: Vec<AllocatedBuffer>,
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

        Self {
            allocator,
            allocated_buffers: Default::default(),
        }
    }

    pub fn upload_mesh(&mut self, mesh: &Mesh) {
        let verticies = &mesh.vertices;
        let buffer_size = std::mem::size_of_val(&mesh.vertices) as u64;

        let buffer_create_info = vk::BufferCreateInfo::default()
            .size(buffer_size)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let allocation_info = vk_mem_alloc::AllocationCreateInfo {
            usage: vk_mem_alloc::MemoryUsage::AUTO,
            flags: vk_mem_alloc::AllocationCreateFlags::HOST_ACCESS_RANDOM,
            ..Default::default()
        };

        let (buffer, allocation, _) = unsafe {
            vk_mem_alloc::create_buffer(self.allocator, &buffer_create_info, &allocation_info)
                .unwrap()
        };

        let mapped_data = unsafe { vk_mem_alloc::map_memory(self.allocator, allocation).unwrap() };

        unsafe {
            std::ptr::copy_nonoverlapping(verticies.as_ptr(), mapped_data as _, verticies.len());
        }

        unsafe { vk_mem_alloc::unmap_memory(self.allocator, allocation) }

        let allocator_buffer =
            buffer::AllocatedBuffer::new(mesh.id, ObjectType::Mesh, buffer, allocation);

        self.allocated_buffers.push(allocator_buffer);
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        unsafe {
            self.allocated_buffers.iter().for_each(|allocated_buffer| {
                vk_mem_alloc::destroy_buffer(
                    self.allocator,
                    allocated_buffer.buffer,
                    allocated_buffer.allocation,
                )
            });

            vk_mem_alloc::destroy_allocator(self.allocator);
        }
    }
}
