use crate::error::{EngineError, EngineResult};
use glium::{
    glutin::surface::WindowSurface, implement_vertex, vertex::VertexBufferAny, Display,
    VertexBuffer,
};
use gltf::{
    buffer::Data,
    mesh::util::{ReadColors, ReadTexCoords},
    Node, Semantic,
};
use vek::{Mat4, Quaternion, Vec3, Vec4};

pub fn get_vertex_data(
    display: &Display<WindowSurface>,
    node: &Node<'_>,
    buffers: &[Data],
) -> EngineResult<Vec<VertexBufferAny>> {
    let mesh = if let Some(mesh) = node.mesh() {
        mesh
    } else {
        return Err(EngineError::GltfError(
            "Node does not have a mesh".to_string(),
        ));
    };

    let (t, r, s) = node.transform().decomposed();

    let translation: Mat4<f32> = Mat4::translation_3d(Vec3::new(t[0], t[1], t[2]));
    let rotation = Mat4::from(Quaternion::from_xyzw(r[0], r[1], r[2], r[3]));
    let scale: Mat4<f32> = Mat4::scaling_3d(Vec3::new(s[0], s[1], s[2]));
    let transform = translation * rotation * scale;

    let mut primitives = Vec::new();
    for primitive in mesh.primitives() {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut tex_coords = Vec::new();
        let mut colors = Vec::new();
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        for (semantic, _) in primitive.attributes() {
            match semantic {
                Semantic::Positions => {
                    for vertex in reader.read_positions().unwrap() {
                        let pos = transform * Vec4::new(vertex[0], vertex[1], vertex[2], 1.0);
                        vertices.push([pos.x, pos.y, pos.z]);
                    }
                }
                Semantic::Normals => {
                    for normal in reader.read_normals().unwrap() {
                        normals.push(normal);
                    }
                }
                Semantic::TexCoords(set) => {
                    let read_tex_coords = if let Some(read_tex_coords) = reader.read_tex_coords(set)
                    {
                        read_tex_coords
                    } else {
                        return Err(EngineError::GltfError(format!(
                            "Couldn't find tex coords in set #{}",
                            set
                        )));
                    };
                    match read_tex_coords {
                        ReadTexCoords::F32(iter) => {
                            for coord in iter {
                                tex_coords.push([coord[0], 1.0 - coord[1]]);
                            }
                        }
                        ReadTexCoords::U16(iter) => {
                            for coord in iter {
                                tex_coords.push([
                                    coord[0] as f32 / u16::MAX as f32,
                                    1.0 - (coord[1] as f32 / u16::MAX as f32),
                                ]);
                            }
                        }
                        ReadTexCoords::U8(iter) => {
                            for coord in iter {
                                tex_coords.push([
                                    coord[0] as f32 / u8::MAX as f32,
                                    1.0 - (coord[1] as f32 / u8::MAX as f32),
                                ]);
                            }
                        }
                    }
                }
                Semantic::Colors(set) => {
                    let read_colors = if let Some(read_color) = reader.read_colors(set) {
                        read_color
                    } else {
                        return Err(EngineError::GltfError(format!(
                            "Couldn't find vertex colors in set #{}",
                            set
                        )));
                    };
                    match read_colors {
                        ReadColors::RgbF32(iter) => {
                            for color in iter {
                                colors.push([color[0], color[1], color[2], 1.0]);
                            }
                        }
                        ReadColors::RgbaF32(iter) => {
                            for color in iter {
                                colors.push(color);
                            }
                        }
                        ReadColors::RgbU16(iter) => {
                            for color in iter {
                                colors.push([
                                    color[0] as f32 / u16::MAX as f32,
                                    color[1] as f32 / u16::MAX as f32,
                                    color[2] as f32 / u16::MAX as f32,
                                    1.0,
                                ]);
                            }
                        }
                        ReadColors::RgbaU16(iter) => {
                            for color in iter {
                                colors.push([
                                    color[0] as f32 / u16::MAX as f32,
                                    color[1] as f32 / u16::MAX as f32,
                                    color[2] as f32 / u16::MAX as f32,
                                    color[3] as f32 / u16::MAX as f32,
                                ]);
                            }
                        }
                        ReadColors::RgbU8(iter) => {
                            for color in iter {
                                colors.push([
                                    color[0] as f32 / u8::MAX as f32,
                                    color[1] as f32 / u8::MAX as f32,
                                    color[2] as f32 / u8::MAX as f32,
                                    1.0,
                                ]);
                            }
                        }
                        ReadColors::RgbaU8(iter) => {
                            for color in iter {
                                colors.push([
                                    color[0] as f32 / u8::MAX as f32,
                                    color[1] as f32 / u8::MAX as f32,
                                    color[2] as f32 / u8::MAX as f32,
                                    color[3] as f32 / u8::MAX as f32,
                                ]);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if vertices.len() != normals.len() || vertices.len() != tex_coords.len() {
            println!("Mismatched attribute counts - vertex = {}, normals = {}, tex coords = {}, vertex colors = {}", vertices.len(), normals.len(), tex_coords.len(), colors.len());
        }

        #[repr(C)]
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 3],
            normal: [f32; 3],
            tex_coords: [f32; 2],
            color: [f32; 4],
        }

        implement_vertex!(Vertex, position, normal, tex_coords, color);

        let mut vertex_data = Vec::new();
        if let Some(indices) = reader.read_indices() {
            for index in indices.into_u32() {
                let index = index as usize;
                let position = vertices[index];
                let normal = normals.get(index).copied().unwrap_or([0.0, 0.0, 1.0]);
                let tex_coords = tex_coords.get(index).copied().unwrap_or([0.0, 0.0]);
                let color = colors.get(index).copied().unwrap_or([1.0, 1.0, 1.0, 1.0]);

                vertex_data.push(Vertex {
                    position,
                    normal,
                    tex_coords,
                    color,
                });
            }
        }
        primitives.push(VertexBuffer::new(display, &vertex_data)?.into());
    }
    Ok(primitives)
}
