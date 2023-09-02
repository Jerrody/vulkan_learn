use arrayvec::ArrayVec;
use ash::vk;

pub struct CommandManager {
    pub command_pool: vk::CommandPool,
    pub command_buffers: ArrayVec<vk::CommandBuffer, 3>,
}

impl CommandManager {
    pub unsafe fn new(device: &ash::Device, queue_family_index: u32, image_count: usize) -> Self {
        let command_pool = unsafe {
            device
                .create_command_pool(
                    &vk::CommandPoolCreateInfo::default()
                        .queue_family_index(queue_family_index)
                        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
                    None,
                )
                .unwrap()
        };

        let command_buffer_alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(image_count as _);

        let command_buffers = unsafe {
            device
                .allocate_command_buffers(&command_buffer_alloc_info)
                .unwrap()
                .into_iter()
                .collect()
        };

        Self {
            command_pool,
            command_buffers,
        }
    }
}
