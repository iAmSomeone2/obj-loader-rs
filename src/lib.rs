#[macro_use]
extern crate lazy_static;

use mesh::MeshData;
use wasm_bindgen::prelude::*;

mod face;
mod mesh;
mod vertex;

#[wasm_bindgen]
pub fn get_default_mesh_data() -> MeshData {
    MeshData::default()
}
