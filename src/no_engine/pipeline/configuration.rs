use crate::no_engine::{shader, Id};

pub mod anti_alising;

#[derive(Default, Hash)]
pub struct PipelineConfiguration {
    pub shaders_id: Vec<Id>,
    pub anti_alising: anti_alising::AntiAliasing,
}

impl PipelineConfiguration {
    #[inline(always)]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline(always)]
    pub fn add_shaders(&mut self, required_shaders: &[shader::Shader]) {
        self.shaders_id.extend_from_slice(
            &required_shaders
                .iter()
                .map(|shader| shader.id)
                .collect::<Vec<_>>(),
        );
    }
}
