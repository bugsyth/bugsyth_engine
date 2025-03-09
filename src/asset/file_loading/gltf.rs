use crate::{
    asset::Model,
    error::{EngineError, EngineResult},
};
use glium::{glutin::surface::WindowSurface, implement_vertex, Display, VertexBuffer};
use gltf::mesh::util::ReadTexCoords;
pub use gltf::*;

/// Vertex structure:
/// struct Vertex {
///     position: [f32; 3],
///     normal: [f32; 3],
///     tex_coords: [f32; 2],
/// }
pub fn load_gltf(
    display: &Display<WindowSurface>,
    path: impl Into<String>,
) -> EngineResult<Vec<Model>> {
    let mut vbos = Vec::new();
    let (gltf, buffers, _) = gltf::import(path.into())?;
    for mesh in gltf.meshes() {
        println!("Mesh #{}", mesh.index());
        for primitive in mesh.primitives() {
            let mut vertices = Vec::new();
            let mut normals = Vec::new();
            let mut tex_coords = Vec::new();
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            println!("- Primitive #{}", primitive.index());
            for (semantic, _) in primitive.attributes() {
                match semantic {
                    Semantic::Positions => {
                        for vertex in reader.read_positions().unwrap() {
                            vertices.push(vertex);
                        }
                    }
                    Semantic::Normals => {
                        for normal in reader.read_normals().unwrap() {
                            normals.push(normal);
                        }
                    }
                    Semantic::TexCoords(set) => {
                        if let Some(read_tex_coords) = reader.read_tex_coords(set) {
                            match read_tex_coords {
                                ReadTexCoords::F32(coords) => {
                                    for coord in coords {
                                        tex_coords.push(coord);
                                    }
                                }
                                _ => {
                                    return Err(EngineError::GltfError(
                                        "U8 and U16 type tex_coords are not supported".to_string(),
                                    ))
                                }
                            }
                        } else {
                            return Err(EngineError::GltfError(
                                "Couldn't find tex coords".to_string(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
            #[derive(Copy, Clone)]
            struct Vertex {
                position: [f32; 3],
                normal: [f32; 3],
                tex_coords: [f32; 2],
            }

            implement_vertex!(Vertex, position, normal, tex_coords);

            let mut data = Vec::new();
            for i in 0..vertices.len() {
                data.push(Vertex {
                    position: vertices[i],
                    normal: normals[i],
                    tex_coords: tex_coords[i],
                });
            }
            vbos.push(Model::new(VertexBuffer::new(display, &data)?.into()));
        }
    }

    Ok(vbos)
}
