use wasm_bindgen::prelude::*;

const FLOAT_SIZE: u32 = 4;

#[wasm_bindgen]
pub struct MeshData {
    vertex_offset: u32,
    /// Number of components used to define a vertex
    vert_component_count: u32,
    /// Number of bytes required for a single vertex component
    vertex_component_size: u32,

    texture_coord_offset: u32,
    /// Number of components used to define a texture coordinate
    texture_coord_component_count: u32,
    /// Number of bytes required for a single texture coordinate component
    texture_coord_component_size: u32,

    normal_offset: u32,
    /// Number of components used to define a vertex normal
    normal_component_count: u32,
    /// Number of bytes required for a single normal component
    normal_component_size: u32,

    /// Raw mesh data vector
    buffer: Vec<u8>,
}

impl Default for MeshData {
    fn default() -> Self {
        Self {
            vertex_offset: 0,
            vert_component_count: 3,
            vertex_component_size: FLOAT_SIZE,
            texture_coord_offset: 3 * FLOAT_SIZE,
            texture_coord_component_count: 2,
            texture_coord_component_size: FLOAT_SIZE,
            normal_offset: 5 * FLOAT_SIZE,
            normal_component_count: 3,
            normal_component_size: FLOAT_SIZE,
            buffer: vec![],
        }
    }
}
