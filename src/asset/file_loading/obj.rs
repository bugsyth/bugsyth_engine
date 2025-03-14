use crate::{asset::model::Model, context::Context, error::EngineResult};
use glium::{VertexBuffer, implement_vertex};

/// Vertex structure:
/// struct Vertex {
///     position: [f32; 3],
///     normal: [f32; 3],
///     tex_coords: [f32; 2],
/// }
pub fn load_wavefront(ctx: &Context, data: &[u8]) -> EngineResult<Model> {
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, normal, tex_coords);

    let mut data = ::std::io::BufReader::new(data);
    let data = obj::ObjData::load_buf(&mut data)?;

    let mut vertex_data = Vec::new();

    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    for v in indices.iter() {
                        let position = data.position[v.0];
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);

                        let tex_coords = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        vertex_data.push(Vertex {
                            position,
                            normal,
                            tex_coords,
                        })
                    }
                }
            }
        }
    }

    Ok(Model::new(
        VertexBuffer::new(&ctx.display, &vertex_data)?.into(),
    ))
}
