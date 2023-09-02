use std::{collections::HashMap, ffi::CStr};

use ash::vk;

use super::Id;

pub struct RawShader {
    pub stage: vk::ShaderStageFlags,
    pub next_stage: vk::ShaderStageFlags,
    pub raw: Vec<u8>,
}

impl RawShader {
    pub fn new(
        shader_stage: vk::ShaderStageFlags,
        next_shader_stage: vk::ShaderStageFlags,
        raw: Vec<u8>,
    ) -> Self {
        Self {
            stage: shader_stage,
            next_stage: next_shader_stage,
            raw,
        }
    }
}

pub struct ShaderObject {
    id: Id,
    stage: vk::ShaderStageFlags,
    shader: vk::ShaderEXT,
}

pub struct ShaderManager<'a> {
    shader_object: ash::extensions::ext::ShaderObject,
    compiler_options: shaderc::CompileOptions<'a>,
    compiler: shaderc::Compiler,
    compiled_shaders: HashMap<Id, RawShader>,
    uploaded_shaders: Vec<ShaderObject>,
    shader_queue_to_load: Vec<Id>,
}

impl ShaderManager<'_> {
    pub const DEFAULT_SHADER_EXTENSION: &'static str = ".glsl";
    pub const DEFAULT_ENTRY_POINT_RAW: &'static CStr =
        unsafe { CStr::from_ptr("main".as_ptr() as _) };
    const DEFAULT_ENTRY_POINT: &'static str = "main";

    pub fn new(instance: &ash::Instance, device: &ash::Device) -> Self {
        let shader_object = ash::extensions::ext::ShaderObject::new(instance, device);

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
            shader_object,
            compiler,
            compiler_options,
            compiled_shaders: Default::default(),
            uploaded_shaders: Default::default(),
            shader_queue_to_load: Default::default(),
        }
    }

    pub fn compile_shaders_from_folder(&mut self, folder_path: &str) {
        let shader_files = std::fs::read_dir(folder_path)
            .unwrap()
            .map(|res: Result<std::fs::DirEntry, std::io::Error>| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        for shader_file in shader_files {
            if !shader_file
                .to_str()
                .unwrap()
                .ends_with(Self::DEFAULT_SHADER_EXTENSION)
            {
                continue;
            }

            let shader_source = std::fs::read_to_string(shader_file.as_path()).unwrap();

            let size = shader_file.metadata().unwrap().len();

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

            self.compile_shader(size, &shader_source, shader_name, shader_type);
        }
    }

    #[inline(always)]
    pub fn compile_shader(
        &mut self,
        _size: u64,
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

        let current_stage = Self::map_shader_stage(shader_type);
        let next_stage = Self::map_next_stage(current_stage);
        let compiled_shader =
            RawShader::new(current_stage, next_stage, spirv.as_binary_u8().to_vec());

        self.compiled_shaders.insert(Id::new(), compiled_shader);
    }

    pub fn upload_required_shaders(&mut self) {
        let shader_infos: Vec<_> = self
            .shader_queue_to_load
            .drain(..)
            .map(|compiled_shader_id| {
                let compiled_shader = self
                    .compiled_shaders
                    .get(&compiled_shader_id)
                    .expect("Shader not found");

                let shader_object = ShaderObject {
                    id: Id::new(),
                    stage: compiled_shader.stage,
                    shader: Default::default(),
                };
                self.uploaded_shaders.push(shader_object);

                vk::ShaderCreateInfoEXT::default()
                    .flags(vk::ShaderCreateFlagsEXT::LINK_STAGE)
                    .stage(compiled_shader.stage)
                    .next_stage(compiled_shader.next_stage)
                    .name(Self::DEFAULT_ENTRY_POINT_RAW)
                    .code(&compiled_shader.raw)
            })
            .collect();

        let uploaded_shaders = unsafe {
            self.shader_object
                .create_shaders(&shader_infos, None)
                .unwrap()
        };

        let start_idx = self.uploaded_shaders.len() - uploaded_shaders.len();
        for (shader_object, uploaded_shader) in self.uploaded_shaders[start_idx..]
            .iter_mut()
            .zip(uploaded_shaders)
        {
            shader_object.shader = uploaded_shader;
        }
    }

    #[inline(always)]
    pub fn clear_shader_modules(&mut self) {
        unsafe {
            for shader_object in self.uploaded_shaders.drain(..) {
                self.shader_object
                    .destroy_shader(shader_object.shader, None);
            }
        }
    }

    #[inline(always)]
    pub fn get_shaders(&self) -> &[ShaderObject] {
        self.uploaded_shaders.as_slice()
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

    #[inline(always)]
    pub fn map_next_stage(shader_stage: vk::ShaderStageFlags) -> vk::ShaderStageFlags {
        match shader_stage {
            vk::ShaderStageFlags::VERTEX => vk::ShaderStageFlags::FRAGMENT,
            vk::ShaderStageFlags::TESSELLATION_CONTROL => {
                vk::ShaderStageFlags::TESSELLATION_EVALUATION
            }
            vk::ShaderStageFlags::TESSELLATION_EVALUATION => vk::ShaderStageFlags::GEOMETRY,
            vk::ShaderStageFlags::GEOMETRY => vk::ShaderStageFlags::FRAGMENT,
            _ => vk::ShaderStageFlags::empty(),
        }
    }
}

impl Drop for ShaderManager<'_> {
    #[inline(always)]
    fn drop(&mut self) {
        self.clear_shader_modules();
    }
}
