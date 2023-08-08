use arrayvec::ArrayVec;
use ash::vk;

pub struct SwapchainManager {
    pub swapchain_loader: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub images: ArrayVec<vk::Image, 3>,
    pub image_views: ArrayVec<vk::ImageView, 3>,
    pub extent: vk::Extent2D,
}

impl SwapchainManager {
    pub fn new(
        instance: &ash::Instance,
        device_manager: &super::device::DeviceManager,
        extent: vk::Extent2D,
        surface: vk::SurfaceKHR,
    ) -> Self {
        let image_count = crate::no_engine::NoEngine::FRAMES_IN_FLIGHT as u32;

        let surface_format = device_manager.surface_format;
        let swapchain_info = vk::SwapchainCreateInfoKHR::default()
            .clipped(true)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .min_image_count(image_count)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(device_manager.present_mode)
            .surface(surface);

        let device = &device_manager.device;
        let swapchain_loader = ash::extensions::khr::Swapchain::new(instance, device);
        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_info, None)
                .unwrap()
        };

        let images = unsafe {
            swapchain_loader
                .get_swapchain_images(swapchain)
                .unwrap()
                .into_iter()
                .collect::<ArrayVec<_, 3>>()
        };

        let image_views = images
            .iter()
            .map(|&image| {
                let image_view_info = vk::ImageViewCreateInfo::default()
                    .image(image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(surface_format.format)
                    .components(vk::ComponentMapping::default())
                    .subresource_range(
                        vk::ImageSubresourceRange::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .base_mip_level(0)
                            .level_count(1)
                            .base_array_layer(0)
                            .layer_count(1),
                    );

                unsafe { device.create_image_view(&image_view_info, None).unwrap() }
            })
            .collect::<ArrayVec<_, 3>>();

        Self {
            swapchain_loader,
            swapchain,
            images,
            image_views,
            extent,
        }
    }
}
