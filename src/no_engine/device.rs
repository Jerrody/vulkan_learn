use ash::vk;

pub struct DeviceManager {
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub queue_family_index: u32,
    pub surface_format: vk::SurfaceFormatKHR,
    pub present_mode: vk::PresentModeKHR,
    pub graphics_queue: vk::Queue,
}

impl DeviceManager {
    pub unsafe fn new(
        instance: &ash::Instance,
        surface_manager: &super::surface::SurfaceManager,
    ) -> Self {
        let surface_loader = &surface_manager.surface_loader;
        let surface = surface_manager.surface;

        let required_extensions = [
            ash::extensions::khr::Swapchain::NAME.as_ptr(),
            ash::extensions::khr::DynamicRendering::NAME.as_ptr(),
            ash::extensions::ext::ShaderObject::NAME.as_ptr(),
            ash::extensions::ext::ExtendedDynamicState::NAME.as_ptr(),
        ];

        let (physical_device, queue_family_index, device_properties, present_mode, surface_format) = unsafe {
            instance
                .enumerate_physical_devices()
                .unwrap()
                .iter()
                .filter_map(|&physical_device| {
                    let Some((queue_family_index, _)) = instance
                        .get_physical_device_queue_family_properties(physical_device)
                        .iter()
                        .enumerate()
                        .find(|(queue_family_index, queue_properties)| {
                            queue_properties
                                .queue_flags
                                .contains(vk::QueueFlags::GRAPHICS)
                                && surface_loader
                                    .get_physical_device_surface_support(
                                        physical_device,
                                        *queue_family_index as _,
                                        surface,
                                    )
                                    .unwrap()
                        })
                    else {
                        return None;
                    };

                    let required_present_modes = [
                        vk::PresentModeKHR::MAILBOX,
                        vk::PresentModeKHR::FIFO_RELAXED,
                    ];
                    let present_modes = surface_loader
                        .get_physical_device_surface_present_modes(physical_device, surface)
                        .unwrap();
                    let does_support_required_present_modes = required_present_modes
                        .iter()
                        .all(|required_present_mode| present_modes.contains(required_present_mode));
                    if !does_support_required_present_modes {
                        return None;
                    }
                    let present_mode = vk::PresentModeKHR::MAILBOX;

                    let surface_formats = surface_loader
                        .get_physical_device_surface_formats(physical_device, surface)
                        .unwrap();
                    let Some(&surface_format) = surface_formats.iter().find(|surface_format| {
                        surface_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
                            && (surface_format.format == vk::Format::B8G8R8A8_SRGB
                                || surface_format.format == vk::Format::R8G8B8A8_SRGB)
                    }) else {
                        return None;
                    };

                    let device_capabilities = surface_loader
                        .get_physical_device_surface_capabilities(physical_device, surface)
                        .unwrap();
                    if device_capabilities.min_image_count < 2
                        || device_capabilities.max_image_count < 3
                    {
                        return None;
                    }

                    let device_extensions = instance
                        .enumerate_device_extension_properties(physical_device)
                        .unwrap();
                    let does_support_required_extensions =
                        required_extensions.iter().all(|&required_extension| {
                            device_extensions.iter().any(|device_extension| {
                                std::ffi::CStr::from_ptr(device_extension.extension_name.as_ptr())
                                    == std::ffi::CStr::from_ptr(required_extension)
                            })
                        });
                    if !does_support_required_extensions {
                        return None;
                    }

                    let device_properties =
                        instance.get_physical_device_properties(physical_device);

                    Some((
                        physical_device,
                        queue_family_index,
                        device_properties,
                        present_mode,
                        surface_format,
                    ))
                })
                .max_by_key(
                    |(_, _, device_properties, _, _)| match device_properties.device_type {
                        vk::PhysicalDeviceType::DISCRETE_GPU => 2,
                        vk::PhysicalDeviceType::INTEGRATED_GPU => 1,
                        _ => Default::default(),
                    },
                )
                .unwrap()
        };

        println!("Found suitable device: {:?}", unsafe {
            std::ffi::CStr::from_ptr(device_properties.device_name.as_ptr())
        });

        let device_queue_info = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index as _)
            .queue_priorities(&[1.0])];

        let physical_device_features = vk::PhysicalDeviceFeatures::default();

        let mut shader_object =
            ash::vk::PhysicalDeviceShaderObjectFeaturesEXT::default().shader_object(true);

        let mut physical_device_vulkan_13_features = vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);

        let mut physical_device_features = vk::PhysicalDeviceFeatures2::default()
            .features(physical_device_features)
            .push_next(&mut physical_device_vulkan_13_features)
            .push_next(&mut shader_object);

        let device_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&device_queue_info)
            .enabled_extension_names(&required_extensions)
            .push_next(&mut physical_device_features);

        let device = unsafe {
            instance
                .create_device(physical_device, &device_info, None)
                .unwrap()
        };

        let graphics_queue =
            unsafe { device.get_device_queue(queue_family_index as _, Default::default()) };

        Self {
            physical_device,
            device,
            queue_family_index: queue_family_index as _,
            surface_format,
            present_mode,
            graphics_queue,
        }
    }
}
