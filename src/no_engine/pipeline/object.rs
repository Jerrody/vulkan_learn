use std::rc::Rc;

use ash::vk;

use super::configuration::PipelineConfiguration;

#[derive(Default, Clone, Copy)]
pub struct PipelineObject {
    pub hash: u64,
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
}

impl PipelineObject {
    pub fn new(
        pipeline: vk::Pipeline,
        pipeline_layout: vk::PipelineLayout,
        pipeline_configuration: Rc<PipelineConfiguration>,
    ) -> Self {
        let hash = crate::no_engine::utils::hash(&pipeline_configuration);

        Self {
            hash,
            pipeline,
            pipeline_layout,
        }
    }
}
