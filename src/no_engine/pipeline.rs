pub mod configuration;

mod builder;
mod object;

use std::collections::HashMap;
use std::rc::Rc;

use ash::vk;

use self::object::PipelineObject;

#[derive(Default)]
pub struct PipelineManager {
    pub current_pipeline_object: Rc<PipelineObject>,
    pipeline_objects: HashMap<u64, PipelineObject>,
    configurations: Vec<configuration::PipelineConfiguration>,
}

impl PipelineManager {
    const DEFAULT_LINE_WIDTH: f32 = 1.0;
    const DEFAULT_MIN_SHADING_SAMPLE: f32 = 1.0;
    const DEFAULT_DEPTH_TEST_RANGE: (f32, f32) = (0.0, 1.0);

    #[inline(always)]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn require_pipeline(
        &mut self,
        device: &ash::Device,
        shader_manager: &super::shader::ShaderManager,
        pipeline_configuration: configuration::PipelineConfiguration,
        extent: vk::Extent2D,
        color_attachment_formats: &[vk::Format],
    ) -> Rc<PipelineObject> {
        let pipeline_configuration_hash = super::utils::hash(&pipeline_configuration);

        if let Some(pipeline_object) = self.pipeline_objects.get(&pipeline_configuration_hash) {
            return Rc::new(*pipeline_object);
        }

        let mut pipeline_builder = builder::PipelineBuilder::new();

        let shaders = shader_manager.get_shaders();
        let required_shaders = pipeline_configuration
            .shaders_id
            .iter()
            .filter_map(|&shader_id| {
                shaders
                    .iter()
                    .find(|shader| shader.id == shader_id)
                    .map(|shader| (shader.shader_module, shader.shader_stage))
            })
            .collect::<Vec<_>>();

        for (shader_module, sahder_stage) in required_shaders {
            pipeline_builder = pipeline_builder.add_shader_stage(shader_module, sahder_stage);
        }

        pipeline_builder = pipeline_builder
            .set_vertex_input_state(Default::default(), Default::default())
            .set_input_assembly_state(vk::PrimitiveTopology::TRIANGLE_LIST, false)
            .set_rasterization_state(
                false,
                false,
                vk::PolygonMode::FILL,
                vk::CullModeFlags::NONE,
                vk::FrontFace::COUNTER_CLOCKWISE,
                false,
                Default::default(),
                Default::default(),
                Default::default(),
                Self::DEFAULT_LINE_WIDTH,
            );

        match pipeline_configuration.anti_alising.anti_aliasing_type {
            configuration::anti_alising::AntiAliasingType::MSAA(_) => {
                let sample_count = pipeline_configuration
                    .anti_alising
                    .anti_aliasing_type
                    .get_sample_count()
                    .unwrap();
                pipeline_builder = pipeline_builder.set_multisample_state(
                    sample_count,
                    false,
                    Self::DEFAULT_MIN_SHADING_SAMPLE,
                    Default::default(),
                    false,
                    false,
                );
            }
            _ => {
                pipeline_builder = pipeline_builder.set_multisample_state(
                    vk::SampleCountFlags::TYPE_1,
                    false,
                    Self::DEFAULT_MIN_SHADING_SAMPLE,
                    Default::default(),
                    false,
                    false,
                );
            }
        }

        pipeline_builder =
            pipeline_builder.set_color_blend_state(false, vk::ColorComponentFlags::RGBA);

        let viewports = [vk::Viewport {
            width: extent.width as f32,
            height: extent.height as f32,
            min_depth: Self::DEFAULT_DEPTH_TEST_RANGE.0,
            max_depth: Self::DEFAULT_DEPTH_TEST_RANGE.1,
            ..Default::default()
        }];
        let scissors = [vk::Rect2D {
            extent: vk::Extent2D {
                width: extent.width,
                height: extent.height,
            },
            ..Default::default()
        }];

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = unsafe {
            device
                .create_pipeline_layout(&pipeline_layout_info, None)
                .expect("Failed to create pipeline layout!")
        };

        pipeline_builder = pipeline_builder.set_layout(pipeline_layout);
        let pipeline =
            pipeline_builder.build(device, &viewports, &scissors, color_attachment_formats);
        let pipeline_object =
            PipelineObject::new(pipeline, pipeline_layout, Rc::new(pipeline_configuration));
        self.pipeline_objects
            .insert(pipeline_configuration_hash, pipeline_object);
        self.current_pipeline_object = Rc::new(pipeline_object);

        self.current_pipeline_object.clone()
    }

    pub unsafe fn clear_pipeline_objects(&mut self, device: &ash::Device) {
        self.pipeline_objects
            .iter()
            .for_each(|(_, pipeline_object)| {
                device.destroy_pipeline(pipeline_object.pipeline, None);
                device.destroy_pipeline_layout(pipeline_object.pipeline_layout, None);
            });
    }
}
