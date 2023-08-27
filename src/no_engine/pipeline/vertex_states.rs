use ash::vk;

use crate::no_engine::objects::mesh::Vertex;

#[derive(Default)]
pub struct VertexInputDescription {
    pub binding_descriptions: Vec<vk::VertexInputBindingDescription>,
    pub attribute_descriptions: Vec<vk::VertexInputAttributeDescription>,
}

impl VertexInputDescription {
    fn new() -> Self {
        Default::default()
    }
}

pub struct VertexStates;

impl VertexStates {
    pub fn get_mesh_vertex_description() -> VertexInputDescription {
        let vertex_input_binding = vk::VertexInputBindingDescription::default()
            .binding(Default::default())
            .input_rate(vk::VertexInputRate::VERTEX)
            .stride(std::mem::size_of::<crate::no_engine::objects::mesh::Vertex>() as u32);

        let mut vertex_input_attributes = Vec::new();
        let mut location = 0;
        vertex_input_attributes.push(
            vk::VertexInputAttributeDescription::default()
                .binding(vertex_input_binding.binding)
                .location(location)
                .format(vk::Format::R32G32B32A32_SFLOAT)
                .offset(std::mem::offset_of!(Vertex, position) as u32),
        );

        location += 1;

        vertex_input_attributes.push(
            vk::VertexInputAttributeDescription::default()
                .binding(vertex_input_binding.binding)
                .location(location)
                .format(vk::Format::R32G32B32A32_SFLOAT)
                .offset(std::mem::offset_of!(Vertex, normal) as u32),
        );

        location += 1;

        vertex_input_attributes.push(
            vk::VertexInputAttributeDescription::default()
                .binding(vertex_input_binding.binding)
                .location(location)
                .format(vk::Format::R32G32B32A32_SFLOAT)
                .offset(std::mem::offset_of!(Vertex, color) as u32),
        );

        let mut vertex_input_description = VertexInputDescription::new();
        vertex_input_description.attribute_descriptions = vertex_input_attributes;
        vertex_input_description
            .binding_descriptions
            .push(vertex_input_binding);

        vertex_input_description
    }
}
