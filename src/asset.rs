mod file_loading;
mod model;
pub use file_loading::{gltf::load_gltf, obj::load_wavefront};
pub use model::Model;

pub struct Asset {
    pub models: Vec<Model>,
}
