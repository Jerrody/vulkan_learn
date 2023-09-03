mod allocator;
mod asset;
mod command;
mod debug_utils;
mod device;
mod id;
mod objects;
mod register;
mod rendering_info;
mod shader;
mod surface;
mod swapchain;
mod utils;

pub use id::*;

use std::{ffi::CString, mem::ManuallyDrop, rc::Rc};

use ash::vk;
use raw_window_handle::HasRawDisplayHandle;

use self::asset::ObjectsQueue;

pub struct NoEngine<'a> {
    entry: ManuallyDrop<ash::Entry>,
    instance: ash::Instance,
    surface_manager: surface::SurfaceManager,
    device_manager: device::DeviceManager,
    shader_manager: shader::ShaderManager<'a>,
    swapchain_manager: swapchain::SwapchainManager,
    debug_handler: debug_utils::DebugHandler,
    command_manager: command::CommandManager,
    render_fence: Rc<[vk::Fence]>,
    rendering_info: rendering_info::RenderingInfo<'static>,
    allocator: allocator::Allocator,
    asset_manager: asset::AssetManager,
    register: register::Register,
    frame_count: u32,
}

impl NoEngine<'_> {
    pub const FRAMES_IN_FLIGHT: usize = 2;

    pub const ENGINE_NAME: &'static str = "No Engine";
    pub const APPLICATION_NAME: &'static str = "Hello Triangle";
    pub const VALIDATION_LAYER_NAME: &'static str = "VK_LAYER_KHRONOS_validation";

    #[inline(always)]
    pub fn new(window: &winit::window::Window) -> Self {
        let entry = ash::Entry::linked();
        let instance = Self::create_instance(window, &entry);
        let debug_handler = debug_utils::DebugHandler::new(&entry, &instance);

        let surface_manager = surface::SurfaceManager::new(&entry, &instance, window);
        let device_manager = unsafe { device::DeviceManager::new(&instance, &surface_manager) };
        let command_manager = unsafe {
            command::CommandManager::new(
                &device_manager.device,
                device_manager.queue_family_index,
                Self::FRAMES_IN_FLIGHT,
            )
        };

        let allocator = allocator::Allocator::new(
            &instance,
            device_manager.physical_device,
            &device_manager.device,
        );

        let window_inner_size = window.inner_size();
        let extent = vk::Extent2D {
            width: window_inner_size.width,
            height: window_inner_size.height,
        };

        let swapchain_manager = swapchain::SwapchainManager::new(
            &instance,
            &device_manager,
            extent,
            surface_manager.surface,
            allocator,
        );

        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        let render_fence = unsafe {
            device_manager
                .device
                .create_fence(&fence_info, None)
                .unwrap()
        };
        let mut shader_manager = shader::ShaderManager::new(&instance, &device_manager.device);
        shader_manager.compile_shaders_from_folder(r"shaders/unlit");

        let semaphore_info = vk::SemaphoreCreateInfo::default();
        let semaphore_rendering = unsafe {
            device_manager
                .device
                .create_semaphore(&semaphore_info, None)
                .unwrap()
        };
        let semaphore_present = unsafe {
            device_manager
                .device
                .create_semaphore(&semaphore_info, None)
                .unwrap()
        };

        let rendering_info = rendering_info::RenderingInfo::new(
            &swapchain_manager,
            &[semaphore_rendering],
            &[semaphore_present],
            &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
        );

        let asset_manager = asset::AssetManager::new();

        Self {
            entry: ManuallyDrop::new(entry),
            instance,
            surface_manager,
            device_manager,
            shader_manager,
            swapchain_manager,
            debug_handler,
            command_manager,
            render_fence: Rc::from([render_fence]),
            rendering_info,
            allocator,
            asset_manager,
            register: register::Register::new(),
            frame_count: Default::default(),
        }
    }

    pub fn create_instance(window: &winit::window::Window, entry: &ash::Entry) -> ash::Instance {
        let version = vk::make_api_version(
            Default::default(),
            Default::default(),
            1,
            Default::default(),
        );

        let application_name = CString::new(Self::APPLICATION_NAME).unwrap();
        let engine_name = CString::new(Self::ENGINE_NAME).unwrap();

        let application_info = vk::ApplicationInfo::default()
            .api_version(vk::API_VERSION_1_3)
            .application_version(version)
            .engine_version(version)
            .application_name(&application_name)
            .engine_name(&engine_name);

        let required_validation_layers = [Self::VALIDATION_LAYER_NAME.as_ptr() as _];
        println!("{}", Self::VALIDATION_LAYER_NAME);
        let mut required_extensions =
            ash_window::enumerate_required_extensions(window.raw_display_handle())
                .unwrap()
                .to_vec();
        required_extensions.push(ash::extensions::ext::DebugUtils::NAME.as_ptr());

        let instance_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_layer_names(&required_validation_layers)
            .enabled_extension_names(&required_extensions);

        unsafe { entry.create_instance(&instance_info, None).unwrap() }
    }

    #[inline(always)]
    pub fn update(&mut self) {
        self.check_upload_queue();
    }

    #[inline(always)]
    pub fn load_file(&mut self, path_buf: std::path::PathBuf) {
        self.asset_manager.load_file(path_buf);
    }

    #[inline(always)]
    fn check_upload_queue(&mut self) {
        let assets_to_upload = self.asset_manager.get_assets_to_upload();
        assets_to_upload
            .iter()
            .for_each(|asset_to_upload| match asset_to_upload {
                ObjectsQueue::Mesh(mesh) => {
                    let mesh = self.asset_manager.get_mesh(*mesh);
                    let allocated_mesh = self.allocator.upload_mesh(mesh);
                    self.register.register_mesh(allocated_mesh);
                }
            });
    }

    #[inline(always)]
    pub fn draw(&mut self) {
        let device = &self.device_manager.device;
        let fences = &self.render_fence;
        unsafe {
            device
                .wait_for_fences(fences, true, u64::MAX)
                .unwrap_unchecked();
            device.reset_fences(fences).unwrap_unchecked();
        };

        let (next_image_index, _) = unsafe {
            self.swapchain_manager
                .swapchain_loader
                .acquire_next_image(
                    self.swapchain_manager.swapchain,
                    u64::MAX,
                    *self
                        .rendering_info
                        .render_semaphores
                        .get_unchecked::<usize>(Default::default()),
                    Default::default(),
                )
                .unwrap_unchecked()
        };

        unsafe {
            device
                .reset_command_pool(
                    self.command_manager.command_pool,
                    vk::CommandPoolResetFlags::RELEASE_RESOURCES,
                )
                .unwrap_unchecked();
        };

        let command_buffer = unsafe {
            *self
                .command_manager
                .command_buffers
                .get_unchecked::<usize>(Default::default())
        };
        unsafe {
            device
                .begin_command_buffer(command_buffer, &self.rendering_info.command_buffer_info)
                .unwrap_unchecked();
        };

        let image = unsafe {
            *self
                .swapchain_manager
                .images
                .get_unchecked::<usize>(next_image_index as _)
        };
        let image_view = unsafe {
            *self
                .swapchain_manager
                .image_views
                .get_unchecked::<usize>(next_image_index as _)
        };
        let queue_family_index = self.device_manager.queue_family_index;

        let color_barrier = vk::ImageMemoryBarrier2 {
            src_stage_mask: vk::PipelineStageFlags2::NONE_KHR,
            src_access_mask: vk::AccessFlags2KHR::NONE_KHR,
            dst_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT_KHR,
            dst_access_mask: vk::AccessFlags2KHR::COLOR_ATTACHMENT_WRITE_KHR,
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            src_queue_family_index: queue_family_index,
            dst_queue_family_index: queue_family_index,
            image,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                level_count: 1,
                layer_count: 1,
                base_array_layer: Default::default(),
                base_mip_level: Default::default(),
            },
            ..Default::default()
        };
        let depth_barrier = vk::ImageMemoryBarrier2 {
            src_stage_mask: vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS
                | vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
            src_access_mask: vk::AccessFlags2KHR::NONE_KHR,
            dst_stage_mask: vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS
                | vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
            dst_access_mask: vk::AccessFlags2KHR::DEPTH_STENCIL_ATTACHMENT_WRITE,
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL,
            src_queue_family_index: queue_family_index,
            dst_queue_family_index: queue_family_index,
            image: self.swapchain_manager.depth.allocated_image.image,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::DEPTH,
                level_count: 1,
                layer_count: 1,
                base_array_layer: Default::default(),
                base_mip_level: Default::default(),
            },
            ..Default::default()
        };
        let output_barrier = vk::ImageMemoryBarrier2 {
            src_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT_KHR,
            src_access_mask: vk::AccessFlags2KHR::COLOR_ATTACHMENT_WRITE_KHR,
            dst_stage_mask: vk::PipelineStageFlags2::NONE_KHR,
            dst_access_mask: vk::AccessFlags2KHR::NONE_KHR,
            old_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            new_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            src_queue_family_index: queue_family_index,
            dst_queue_family_index: queue_family_index,
            image,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                level_count: 1,
                layer_count: 1,
                base_mip_level: Default::default(),
                base_array_layer: Default::default(),
            },
            ..Default::default()
        };
        let image_barriers = [color_barrier, depth_barrier, output_barrier];
        let dependency_info =
            vk::DependencyInfoKHR::default().image_memory_barriers(&image_barriers);
        unsafe { device.cmd_pipeline_barrier2(command_buffer, &dependency_info) };

        let clear_values = unsafe {
            self.rendering_info
                .clear_values
                .color
                .float32
                .get_unchecked_mut(2)
        };
        *clear_values = f32::abs(self.frame_count as f32 / 100.0 % 1.0 - 0.5) * 2.0;

        let color_attachment = unsafe {
            self.rendering_info
                .color_attachments
                .get_unchecked_mut::<usize>(Default::default())
        };
        *color_attachment = color_attachment
            .clear_value(self.rendering_info.clear_values)
            .image_view(image_view);

        let rendering_info = vk::RenderingInfoKHR::default()
            .color_attachments(&self.rendering_info.color_attachments)
            .depth_attachment(&self.rendering_info.depth_attachment)
            .render_area(vk::Rect2D {
                offset: Default::default(),
                extent: self.swapchain_manager.extent,
            })
            .layer_count(1);

        unsafe {
            device.cmd_begin_rendering(command_buffer, &rendering_info);

            self.register.get_meshes().iter().for_each(|mesh| {
                device.cmd_bind_vertex_buffers(
                    command_buffer,
                    Default::default(),
                    self.register.get_buffers(),
                    self.register.get_offsets(),
                );
                device.cmd_bind_index_buffer(
                    command_buffer,
                    mesh.index_buffer.buffer,
                    Default::default(),
                    vk::IndexType::UINT32,
                );

                let metadata = mesh.metadata;
                device.cmd_draw_indexed(
                    command_buffer,
                    metadata.indices_count,
                    1,
                    Default::default(),
                    Default::default(),
                    Default::default(),
                );
            });

            device.cmd_end_rendering(command_buffer);
        }

        unsafe {
            device.end_command_buffer(command_buffer).unwrap_unchecked();
        }

        let command_buffers = [command_buffer];

        let submit_infos = [vk::SubmitInfo::default()
            .command_buffers(&command_buffers)
            .wait_semaphores(&self.rendering_info.render_semaphores)
            .wait_dst_stage_mask(&self.rendering_info.wait_dst_stage_mask)
            .signal_semaphores(&self.rendering_info.present_semaphores)];

        unsafe {
            device
                .queue_submit(
                    self.device_manager.graphics_queue,
                    &submit_infos,
                    *fences.get_unchecked::<usize>(Default::default()),
                )
                .unwrap_unchecked();
        }

        let image_indices = [next_image_index];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&self.rendering_info.present_semaphores)
            .swapchains(&self.rendering_info.swapchains)
            .image_indices(&image_indices);

        unsafe {
            self.swapchain_manager
                .swapchain_loader
                .queue_present(self.device_manager.graphics_queue, &present_info)
                .unwrap_unchecked();
        };

        self.frame_count += 1;
    }
}

impl Drop for NoEngine<'_> {
    fn drop(&mut self) {
        unsafe {
            let device = &self.device_manager.device;
            device.device_wait_idle().unwrap();

            self.shader_manager.clear_uploaded_shaders();
            device.destroy_command_pool(self.command_manager.command_pool, None);

            let swapchain_manager = &self.swapchain_manager;

            swapchain_manager
                .image_views
                .iter()
                .for_each(|&image_view| {
                    device.destroy_image_view(image_view, None);
                });

            swapchain_manager
                .swapchain_loader
                .destroy_swapchain(swapchain_manager.swapchain, None);

            device.destroy_fence(*self.render_fence.first().unwrap(), None);
            self.rendering_info
                .render_semaphores
                .iter()
                .zip(self.rendering_info.present_semaphores.iter())
                .for_each(|(&render_semaphore, &present_semaphore)| {
                    device.destroy_semaphore(render_semaphore, None);
                    device.destroy_semaphore(present_semaphore, None);
                });

            device.destroy_device(None);

            let debug_handler = &self.debug_handler;
            debug_handler
                .debug_loader
                .destroy_debug_utils_messenger(debug_handler.debug_messenger, None);
            self.surface_manager
                .surface_loader
                .destroy_surface(self.surface_manager.surface, None);
            self.instance.destroy_instance(None);

            ManuallyDrop::drop(&mut self.entry);
        }
    }
}
