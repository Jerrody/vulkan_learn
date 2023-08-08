use ash::vk;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub struct SurfaceManager {
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: vk::SurfaceKHR,
}

impl SurfaceManager {
    pub fn new(
        entry: &ash::Entry,
        instance: &ash::Instance,
        window: &winit::window::Window,
    ) -> Self {
        let surface_loader = ash::extensions::khr::Surface::new(entry, instance);
        let surface = unsafe {
            ash_window::create_surface(
                entry,
                instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )
            .unwrap()
        };
        Self {
            surface_loader,
            surface,
        }
    }
}
