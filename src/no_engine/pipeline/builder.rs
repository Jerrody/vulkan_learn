use ash::vk;

#[derive(Default)]
pub struct PipelineBuilder<'a> {
    shader_stages: Vec<vk::PipelineShaderStageCreateInfo<'a>>,
    vertex_input_state: vk::PipelineVertexInputStateCreateInfo<'a>,
    input_assembly_state: vk::PipelineInputAssemblyStateCreateInfo<'a>,
    viewport_state: vk::PipelineViewportStateCreateInfo<'a>,
    rasterization_state: vk::PipelineRasterizationStateCreateInfo<'a>,
    multisample_state: vk::PipelineMultisampleStateCreateInfo<'a>,
    depth_stencil_state: vk::PipelineDepthStencilStateCreateInfo<'a>,
    color_blend_attachment_states: Vec<vk::PipelineColorBlendAttachmentState>,
    dynamic_state: vk::PipelineDynamicStateCreateInfo<'a>,
    pipeline_layout: vk::PipelineLayout,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_shader_stage(
        mut self,
        shader_module: vk::ShaderModule,
        shader_stage: vk::ShaderStageFlags,
    ) -> Self {
        let entry_point = unsafe {
            std::ffi::CStr::from_ptr(
                crate::no_engine::shader::ShaderManager::DEFAULT_ENTRY_POINT_RAW,
            )
        };

        let shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(shader_stage)
            .module(shader_module)
            .name(entry_point);

        self.shader_stages.push(shader_stage_info);

        self
    }

    pub fn set_vertex_input_state(
        mut self,
        vertex_binding_descriptions: &'a [vk::VertexInputBindingDescription],
        vertex_attribute_descriptions: &'a [vk::VertexInputAttributeDescription],
    ) -> Self {
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(vertex_binding_descriptions)
            .vertex_attribute_descriptions(vertex_attribute_descriptions);

        self.vertex_input_state = vertex_input_state;

        self
    }

    pub fn set_input_assembly_state(
        mut self,
        topology: vk::PrimitiveTopology,
        primitive_restart_enable: bool,
    ) -> Self {
        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(topology)
            .primitive_restart_enable(primitive_restart_enable);

        self.input_assembly_state = input_assembly_state;

        self
    }

    pub fn set_viewport_state(
        mut self,
        viewports: &'a [vk::Viewport],
        scissors: &'a [vk::Rect2D],
    ) -> Self {
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(viewports)
            .scissors(scissors);

        self.viewport_state = viewport_state;

        self
    }

    pub fn set_rasterization_state(
        mut self,
        depth_clamp_enable: bool,
        rasterizer_discard_enable: bool,
        polygon_mode: vk::PolygonMode,
        cull_mode: vk::CullModeFlags,
        front_face: vk::FrontFace,
        depth_bias_enable: bool,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
        line_width: f32,
    ) -> Self {
        let rasterization_state = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(depth_clamp_enable)
            .rasterizer_discard_enable(rasterizer_discard_enable)
            .polygon_mode(polygon_mode)
            .cull_mode(cull_mode)
            .front_face(front_face)
            .depth_bias_enable(depth_bias_enable)
            .depth_bias_constant_factor(depth_bias_constant_factor)
            .depth_bias_clamp(depth_bias_clamp)
            .depth_bias_slope_factor(depth_bias_slope_factor)
            .line_width(line_width);

        self.rasterization_state = rasterization_state;

        self
    }

    pub fn set_multisample_state(
        mut self,
        rasterization_samples: vk::SampleCountFlags,
        sample_shading_enable: bool,
        min_sample_shading: f32,
        sample_mask: &'a [vk::SampleMask],
        alpha_to_coverage_enable: bool,
        alpha_to_one_enable: bool,
    ) -> Self {
        let multisample_state = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(rasterization_samples)
            .sample_shading_enable(sample_shading_enable)
            .min_sample_shading(min_sample_shading)
            .sample_mask(sample_mask)
            .alpha_to_coverage_enable(alpha_to_coverage_enable)
            .alpha_to_one_enable(alpha_to_one_enable);

        self.multisample_state = multisample_state;

        self
    }

    pub fn set_depth_stencil_state(
        mut self,
        depth_test_enable: bool,
        depth_write_enable: bool,
        depth_compare_op: vk::CompareOp,
        depth_bounds_test_enable: bool,
        stencil_test_enable: bool,
        front: vk::StencilOpState,
        back: vk::StencilOpState,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) -> Self {
        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(depth_test_enable)
            .depth_write_enable(depth_write_enable)
            .depth_compare_op(depth_compare_op)
            .depth_bounds_test_enable(depth_bounds_test_enable)
            .stencil_test_enable(stencil_test_enable)
            .front(front)
            .back(back)
            .min_depth_bounds(min_depth_bounds)
            .max_depth_bounds(max_depth_bounds);

        self.depth_stencil_state = depth_stencil_state;

        self
    }

    pub fn set_color_blend_state(
        mut self,
        blend_enable: bool,
        color_write_mask: vk::ColorComponentFlags,
    ) -> Self {
        let color_blend_state = vk::PipelineColorBlendAttachmentState::default()
            .blend_enable(blend_enable)
            .color_write_mask(color_write_mask);

        self.color_blend_attachment_states.push(color_blend_state);

        self
    }

    pub fn set_dynamic_state(mut self, dynamic_states: &'a [vk::DynamicState]) -> Self {
        let dynamic_state =
            vk::PipelineDynamicStateCreateInfo::default().dynamic_states(dynamic_states);

        self.dynamic_state = dynamic_state;

        self
    }

    pub fn set_layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.pipeline_layout = layout;

        self
    }

    pub fn build(
        self,
        device: &ash::Device,
        color_attachment_formats: &[vk::Format],
    ) -> vk::Pipeline {
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::default()
            .attachments(&self.color_blend_attachment_states)
            .logic_op(vk::LogicOp::COPY);

        let mut pipeline_rendering_info = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(color_attachment_formats);

        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&self.shader_stages)
            .vertex_input_state(&self.vertex_input_state)
            .input_assembly_state(&self.input_assembly_state)
            .viewport_state(&self.viewport_state)
            .rasterization_state(&self.rasterization_state)
            .multisample_state(&self.multisample_state)
            .depth_stencil_state(&self.depth_stencil_state)
            .color_blend_state(&color_blend_state)
            .viewport_state(&self.viewport_state)
            .dynamic_state(&self.dynamic_state)
            .layout(self.pipeline_layout)
            .push_next(&mut pipeline_rendering_info);

        unsafe {
            device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
                .unwrap()
                .first()
                .unwrap_unchecked()
                .to_owned()
        }
    }
}
