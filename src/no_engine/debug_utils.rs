use ash::vk;

unsafe extern "system" fn debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message_severity = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "VERBOSE",
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "WARNING",
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "ERROR",
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "INFO",
        _ => "UNKNOWN",
    };
    let message_type = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "GENERAL",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "PERFORMANCE",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "VALIDATION",
        _ => "UNKNOWN",
    };
    let message = unsafe { std::ffi::CStr::from_ptr((*p_callback_data).p_message) };
    println!("[DEBUG] [{message_severity}] [{message_type}] {message:?}");
    vk::FALSE
}

pub struct DebugHandler {
    pub debug_loader: ash::extensions::ext::DebugUtils,
    pub debug_messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugHandler {
    pub fn new(entry: &ash::Entry, instance: &ash::Instance) -> Self {
        let debug_loader = ash::extensions::ext::DebugUtils::new(entry, instance);
        let debug_messenger = unsafe {
            debug_loader
                .create_debug_utils_messenger(
                    &vk::DebugUtilsMessengerCreateInfoEXT::default()
                        .message_severity(
                            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                        )
                        .message_type(
                            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                        )
                        .pfn_user_callback(Some(debug_callback)),
                    None,
                )
                .unwrap()
        };

        Self {
            debug_loader,
            debug_messenger,
        }
    }
}
