mod file_loading;
pub mod model;
pub use file_loading::{gltf::load_gltf, obj::load_wavefront};
mod animation;
pub mod skeleton;

use animation::Animations;
use model::Model;
use skeleton::Skeleton;

pub struct Asset {
    pub model: Model,
    pub mesh_type: MeshType,
    pub skeleton: Option<Skeleton>,
    pub animations: Option<Animations>,
}

impl Asset {
    pub fn new(
        model: Model,
        mesh_type: MeshType,
        skeleton: Option<Skeleton>,
        animations: Option<Animations>,
    ) -> Self {
        Self {
            model,
            mesh_type,
            skeleton,
            animations,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MeshType {
    Standard,
    Skeletal,
}
