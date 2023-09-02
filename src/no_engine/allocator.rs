pub mod buffer;
pub mod image;
pub mod mesh;

pub use self::buffer::*;
pub use self::image::*;

use self::buffer::AllocatedBuffer;
use self::mesh::AllocatedMesh;

use super::objects::ObjectType;
use super::Id;
use ash::vk;

use super::objects::mesh::Mesh;

#[derive(Clone, Copy)]
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

    #[inline(always)]
    pub fn upload_mesh(&mut self, mesh: &Mesh) -> AllocatedMesh {
        let vertex_buffer = self.allocate_buffer(
            &mesh.vertices,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::SharingMode::EXCLUSIVE,
        );

        let index_buffer = self.allocate_buffer(
            &mesh.indices,
            vk::BufferUsageFlags::INDEX_BUFFER,
            vk::SharingMode::EXCLUSIVE,
        );

        AllocatedMesh::new(mesh.metadata.id, mesh.metadata, vertex_buffer, index_buffer)
    }

    #[inline(always)]
    pub fn allocate_image(
        &self,
        format: vk::Format,
        extent: vk::Extent3D,
        image_type: vk::ImageType,
        array_layers: u32,
        mip_map_levels: u32,
        samples: vk::SampleCountFlags,
        usage_flags: vk::ImageUsageFlags,
        flags: vk_mem_alloc::AllocationCreateFlags,
        required_flags: vk::MemoryPropertyFlags,
    ) -> AllocatedImage {
        let image_create_info = vk::ImageCreateInfo::default()
            .array_layers(array_layers)
            .mip_levels(mip_map_levels)
            .image_type(image_type)
            .extent(extent)
            .format(format)
            .samples(samples)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(usage_flags);

        let allocation_info = vk_mem_alloc::AllocationCreateInfo {
            usage: vk_mem_alloc::MemoryUsage::AUTO,
            flags,
            required_flags,
            ..Default::default()
        };

        let (image, allocation, _) = unsafe {
            vk_mem_alloc::create_image(self.allocator, &image_create_info, &allocation_info)
                .unwrap()
        };

        AllocatedImage::new(Id::new(), format, image, allocation)
    }

    pub fn allocate_buffer<T>(
        &self,
        data: &[T],
        usage: vk::BufferUsageFlags,
        sharing: vk::SharingMode,
    ) -> AllocatedBuffer {
        let buffer_size = std::mem::size_of_val(data) as u64;

        let buffer_create_info = vk::BufferCreateInfo::default()
            .size(buffer_size)
            .usage(usage)
            .sharing_mode(sharing);

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
            std::ptr::copy_nonoverlapping(data.as_ptr(), mapped_data as _, data.len());
        }

        unsafe { vk_mem_alloc::unmap_memory(self.allocator, allocation) }

        buffer::AllocatedBuffer::new(Id::new(), buffer_size, ObjectType::Mesh, buffer, allocation)
    }

    #[inline(always)]
    pub fn destroy_allocator(&mut self) {
        unsafe {
            vk_mem_alloc::destroy_allocator(self.allocator);
        }
    }
}
