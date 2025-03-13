mod file_loading;
pub mod model;
pub use file_loading::obj::load_wavefront;
use model::Model;

pub struct Asset {
    pub model: Model,
    pub mesh_type: MeshType,
}

impl Asset {
    pub fn new(model: Model, mesh_type: MeshType) -> Self {
        Self { model, mesh_type }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MeshType {
    /// position: [f32; 3],
    /// normal: [f32; 3],
    /// tex_coords: [f32; 2],
    /// color: [f32; 4],
    Standard,
    /// position: [f32; 3],
    /// normal: [f32; 3],
    /// tex_coords: [f32; 2],
    /// color: [f32; 4],
    /// joint_set: [u16; 4],
    /// weights: [f32; 4],
    Skeletal,
}
