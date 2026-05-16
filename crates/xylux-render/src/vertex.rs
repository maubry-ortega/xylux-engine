use ash::vk;
use std::mem::offset_of;
pub use xylux_core::Vertex;

pub fn get_vertex_binding_description() -> vk::VertexInputBindingDescription {
    vk::VertexInputBindingDescription {
        binding: 0,
        stride: std::mem::size_of::<Vertex>() as u32,
        input_rate: vk::VertexInputRate::VERTEX,
    }
}

pub fn get_vertex_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 3] {
    [
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex, pos) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 1,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex, color) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 2,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex, normal) as u32,
        },
    ]
}
