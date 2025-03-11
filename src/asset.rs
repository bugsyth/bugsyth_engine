mod file_loading;
pub mod model;
pub use file_loading::{gltf::load_gltf, obj::load_wavefront};
pub mod skeleton;

use model::Model;
use skeleton::Skeleton;

pub struct Asset {
    pub model: Model,
    pub mesh_type: MeshType,
    pub skeleton: Option<Skeleton>,
}

impl Asset {
    pub fn new(model: Model, skeleton: Option<Skeleton>, mesh_type: MeshType) -> Self {
        Self {
            model,
            mesh_type,
            skeleton,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MeshType {
    Standard,
    Skeletal,
}
