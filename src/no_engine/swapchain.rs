use arrayvec::ArrayVec;
use ash::vk;

use super::allocator::{self, Allocator};

pub struct Depth {
    pub image_view: vk::ImageView,
    pub allocated_image: allocator::AllocatedImage,
}

impl Depth {
    pub fn new(image_view: vk::ImageView, allocated_image: allocator::AllocatedImage) -> Self {
        Self {
            image_view,
            allocated_image,
        }
    }
}

pub struct SwapchainManager {
    pub swapchain_loader: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub images: ArrayVec<vk::Image, 3>,
    pub image_views: ArrayVec<vk::ImageView, 3>,
    pub depth: Depth,
    pub extent: vk::Extent2D,
}

impl SwapchainManager {
    pub fn new(
        instance: &ash::Instance,
        device_manager: &super::device::DeviceManager,
        extent: vk::Extent2D,
        surface: vk::SurfaceKHR,
        allocator: Allocator,
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

        let allocated_depth_image = allocator.allocate_image(
            vk::Format::D32_SFLOAT,
            vk::Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            },
            vk::ImageType::TYPE_2D,
            1,
            1,
            vk::SampleCountFlags::TYPE_1,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            Default::default(),
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );

        let depth_image_view_info = vk::ImageViewCreateInfo {
            image: allocated_depth_image.image,
            view_type: vk::ImageViewType::TYPE_2D,
            format: allocated_depth_image.format,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::DEPTH,
                level_count: 1,
                layer_count: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        let depth_image_view = unsafe {
            device_manager
                .device
                .create_image_view(&depth_image_view_info, None)
                .unwrap()
        };

        let depth = Depth::new(depth_image_view, allocated_depth_image);

        Self {
            swapchain_loader,
            swapchain,
            images,
            image_views,
            extent,
            depth,
        }
    }
}
