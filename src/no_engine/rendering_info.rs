use std::rc::Rc;

use ash::vk;

use super::allocator;

pub struct Depth {
    image_view: vk::ImageView,
    allocated_image: allocator::AllocatedImage,
}

pub struct RenderingInfo<'a> {
    pub command_buffer_info: vk::CommandBufferBeginInfo<'a>,
    pub color_attachments: arrayvec::ArrayVec<vk::RenderingAttachmentInfoKHR<'a>, 1>,
    pub clear_values: vk::ClearValue,
    pub present_semaphores: Rc<[vk::Semaphore]>,
    pub render_semaphores: Rc<[vk::Semaphore]>,
    pub wait_dst_stage_mask: Rc<[vk::PipelineStageFlags]>,
    pub swapchains: Rc<[vk::SwapchainKHR]>,
}

impl RenderingInfo<'_> {
    pub fn new(
        swapchain_manager: &super::swapchain::SwapchainManager,
        wait_semaphores: &[vk::Semaphore],
        signal_semaphores: &[vk::Semaphore],
        wait_dst_stage_mask: &[vk::PipelineStageFlags],
    ) -> Self {
        let command_buffer_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        let color_attachments = [vk::RenderingAttachmentInfoKHR::default()
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

        let clear_values = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [Default::default(); 4],
            },
        };

        let wait_semaphores = Rc::from(wait_semaphores);
        let signal_semaphores = Rc::from(signal_semaphores);
        let wait_dst_stage_mask = Rc::from(wait_dst_stage_mask);
        let swapchains = Rc::from([swapchain_manager.swapchain]);

        Self {
            command_buffer_info,
            color_attachments: color_attachments.into_iter().collect(),
            clear_values,
            present_semaphores: wait_semaphores,
            render_semaphores: signal_semaphores,
            wait_dst_stage_mask,
            swapchains,
        }
    }
}
