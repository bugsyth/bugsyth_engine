use crate::{
    asset::{skeleton::Skeleton, Asset, Model},
    error::EngineResult,
};
use glium::{glutin::surface::WindowSurface, Display};
mod mesh_loader;
mod skeleton_loader;

/// Vertex structure:
/// struct StandardVertex {
///     position: [f32; 3],
///     normal: [f32; 3],
///     tex_coords: [f32; 2],
///     color: [f32; 4],
/// }
///
/// struct SkeletalVertex {
///     position: [f32; 3],
///     normal: [f32; 3],
///     tex_coords: [f32; 2],
///     color: [f32; 4],
///     joint_set: [u16; 4],
///     weights: [f32; 4],
// }
pub fn load_gltf(
    display: &Display<WindowSurface>,
    path: impl Into<String>,
) -> EngineResult<Vec<Asset>> {
    let (gltf, buffers, _) = gltf::import(path.into())?;
    let animations = gltf.animations();
    let skins = gltf.skins();
    let nodes = gltf.nodes();

    for animation in animations {
        println!("{:?}", animation.name());
    }

    let mut assets = Vec::new();
    let mut models = Vec::new();
    let skeletons: Option<Vec<Skeleton>> = if skins.len() > 0 {
        Some(
            skins
                .map(|skin| skeleton_loader::build_skeleton(skin, &buffers))
                .collect(),
        )
    } else {
        None
    };

    for node in nodes {
        // Only using the first primitive, no support for multiple yet
        let mut vertex_data = mesh_loader::get_vertex_data(display, &node, &buffers)?;
        if vertex_data.len() > 0 {
            let (mesh_type, vbo) = vertex_data.remove(0);
            models.push((Model::new(vbo), mesh_type));
        }
    }

    let (model, gltf_mesh_type) = models.remove(0);
    assets.push(Asset::new(
        model,
        if let Some(vec) = skeletons {
            Some(vec[0].clone())
        } else {
            None
        },
        gltf_mesh_type,
    ));

    Ok(assets)
}
