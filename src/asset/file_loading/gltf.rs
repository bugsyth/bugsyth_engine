use crate::{asset::Model, error::EngineResult};
use glium::{glutin::surface::WindowSurface, Display};

mod vertex;

/// Vertex structure:
/// struct Vertex {
///     position: [f32; 3],
///     normal: [f32; 3],
///     tex_coords: [f32; 2],
///     color: [f32; 4],
/// }
pub fn load_gltf(
    display: &Display<WindowSurface>,
    path: impl Into<String>,
) -> EngineResult<Vec<Model>> {
    let mut models = Vec::new();
    let (gltf, buffers, _) = gltf::import(path.into())?;
    for node in gltf.nodes() {
        if node.mesh().is_none() {
            continue;
        }
        // Only using the first primitive, no support for multiple yet
        let mut vertex_data = vertex::get_vertex_data(display, &node, &buffers)?;
        let vbo = vertex_data.remove(0);
        models.push(Model::new(vbo));
    }

    Ok(models)
}
