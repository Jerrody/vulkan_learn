use std::sync::Arc;

use ash::vk;

use super::Id;

pub struct Shader {
    pub id: Id,
    pub shader_module: vk::ShaderModule,
    pub shader_stage: vk::ShaderStageFlags,
}

impl Shader {
    pub fn new(shader_module: vk::ShaderModule, shader_stage: vk::ShaderStageFlags) -> Self {
        Self {
            id: Id::new(),
            shader_module,
            shader_stage,
        }
    }
}

pub struct ShaderManager<'a> {
    device: Arc<ash::Device>,
    compiler: shaderc::Compiler,
    compiler_options: shaderc::CompileOptions<'a>,
    shader_modules: Vec<Shader>,
}

impl ShaderManager<'_> {
    pub const DEFAULT_SHADER_EXTENSION: &'static str = ".glsl";
    pub const DEFAULT_ENTRY_POINT_RAW: *const std::os::raw::c_char = concat!("main", "\0")
        .as_ptr()
        .cast::<::std::os::raw::c_char>();
    const DEFAULT_ENTRY_POINT: &'static str = "main";

    pub fn new(device: &ash::Device) -> Self {
        let compiler = shaderc::Compiler::new().unwrap();
        let mut compiler_options = shaderc::CompileOptions::new().unwrap();
        compiler_options.set_target_env(
            shaderc::TargetEnv::Vulkan,
            shaderc::EnvVersion::Vulkan1_3 as _,
        );

        compiler_options.set_optimization_level(shaderc::OptimizationLevel::Performance);
        compiler_options.set_source_language(shaderc::SourceLanguage::GLSL);
        compiler_options.set_target_spirv(shaderc::SpirvVersion::V1_6);
        compiler_options.set_warnings_as_errors();

        Self {
            compiler,
            compiler_options,
            device: Arc::new(device.clone()),
            shader_modules: Default::default(),
        }
    }

    pub fn compile_shaders_from_folder(&mut self, folder_path: &str) {
        let shader_files = std::fs::read_dir(folder_path)
            .unwrap()
            .map(|res: Result<std::fs::DirEntry, std::io::Error>| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        for shader_file in shader_files {
            let shader_source = std::fs::read_to_string(shader_file.as_path()).unwrap();
            if !shader_file
                .as_path()
                .to_str()
                .unwrap()
                .ends_with(Self::DEFAULT_SHADER_EXTENSION)
            {
                continue;
            }

            let mut shader_file_split = shader_file
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split('.');

            let shader_name = shader_file_split.next().unwrap();
            let shader_type_raw = shader_file_split.next().unwrap();

            let shader_type = match shader_type_raw {
                "vert" => shaderc::ShaderKind::Vertex,
                "frag" => shaderc::ShaderKind::Fragment,
                "comp" => shaderc::ShaderKind::Compute,
                unknown_shader_type => panic!(
                    "{}",
                    std::format!("Unsupported shader type: {unknown_shader_type}")
                ),
            };

            self.compile_shader(&shader_source, shader_name, shader_type);
        }
    }

    #[inline(always)]
    pub fn compile_shader(
        &mut self,
        shader_source: &str,
        shader_name: &str,
        shader_type: shaderc::ShaderKind,
    ) {
        let spirv = self
            .compiler
            .compile_into_spirv(
                shader_source,
                shader_type,
                shader_name,
                Self::DEFAULT_ENTRY_POINT,
                Some(&self.compiler_options),
            )
            .unwrap();

        let shader_code =
            ash::util::read_spv(&mut std::io::Cursor::new(&spirv.as_binary_u8())).unwrap();

        let shader_module = unsafe { self.create_shader_module(&shader_code) };

        let shader = Shader::new(shader_module, Self::map_shader_stage(shader_type));
        self.shader_modules.push(shader);
    }

    #[inline(always)]
    unsafe fn create_shader_module(&self, spirv: &[u32]) -> vk::ShaderModule {
        let create_info = vk::ShaderModuleCreateInfo::default().code(spirv);

        unsafe {
            self.device
                .create_shader_module(&create_info, None)
                .expect("Failed to create shader module!")
        }
    }

    #[inline(always)]
    pub fn clear_shader_modules(&mut self) {
        unsafe {
            for shader_module in self.shader_modules.drain(..) {
                self.device
                    .destroy_shader_module(shader_module.shader_module, None);
            }
        }
    }

    #[inline(always)]
    pub fn get_shaders(&self) -> &[Shader] {
        &self.shader_modules
    }

    #[inline(always)]
    fn map_shader_stage(shader_type: shaderc::ShaderKind) -> vk::ShaderStageFlags {
        match shader_type {
            shaderc::ShaderKind::Vertex => vk::ShaderStageFlags::VERTEX,
            shaderc::ShaderKind::Fragment => vk::ShaderStageFlags::FRAGMENT,
            shaderc::ShaderKind::Compute => vk::ShaderStageFlags::COMPUTE,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

impl Drop for ShaderManager<'_> {
    #[inline(always)]
    fn drop(&mut self) {
        self.clear_shader_modules();
    }
}
