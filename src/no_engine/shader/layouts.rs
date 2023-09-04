use ash::vk;

pub struct ShaderBinding<'a> {
    pub binding_description: vk::VertexInputBindingDescription2EXT<'a>,
    pub attribute_descriptions: Vec<vk::VertexInputAttributeDescription2EXT<'a>>,
}

pub struct ShaderLayout<'a> {
    pub bindings: Vec<ShaderBinding<'a>>,
}

impl ShaderLayout<'_> {
    pub fn new() -> Self {
        Self {
            bindings: Default::default(),
        }
    }
}
