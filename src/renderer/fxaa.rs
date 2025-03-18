use crate::{
    context::Context,
    error::{EngineError, EngineResult},
    renderer::{Drawable, Renderer},
    shaders::{FXAA_FS, FXAA_VS},
};
use glium::{
    DrawParameters, IndexBuffer, Surface, Texture2d, VertexBuffer,
    framebuffer::{DepthRenderBuffer, SimpleFrameBuffer},
    implement_vertex,
    index::{IndicesSource, PrimitiveType},
    program,
    texture::DepthFormat,
    uniform,
    uniforms::Uniforms,
    vertex::MultiVerticesSource,
};

/// FXAA program needs to be send to the context
/// Create a FXAARenderer to use FXAA
pub struct FXAA {
    vbo: VertexBuffer<Vertex>,
    ibo: IndexBuffer<u16>,
    color: Texture2d,
    depth: DepthRenderBuffer,
}

impl FXAA {
    pub fn new(ctx: &Context, width: u32, height: u32) -> EngineResult<Self> {
        Ok(Self {
            vbo: VertexBuffer::new(
                &ctx.display,
                &[
                    Vertex {
                        position: [-1.0, -1.0],
                        i_tex_coords: [0.0, 0.0],
                    },
                    Vertex {
                        position: [-1.0, 1.0],
                        i_tex_coords: [0.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, 1.0],
                        i_tex_coords: [1.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, -1.0],
                        i_tex_coords: [1.0, 0.0],
                    },
                ],
            )?,
            ibo: IndexBuffer::new(&ctx.display, PrimitiveType::TriangleStrip, &[1, 2, 0, 3u16])?,
            color: Texture2d::empty(&ctx.display, width, height)?,
            depth: DepthRenderBuffer::new(&ctx.display, DepthFormat::I24, width, height)?,
        })
    }

    /// Sends a program call 'fxaa' to the Context
    pub fn send_program(ctx: &mut Context) -> EngineResult {
        ctx.add_program(
            "fxaa",
            program!(&ctx.display,
                100 => {
                    vertex: FXAA_VS,
                    fragment: FXAA_FS,
                },
            )?,
        );
        Ok(())
    }
}

impl Drawable for FXAA {
    fn get_vbo(&self) -> impl MultiVerticesSource {
        &self.vbo
    }
    fn get_ibo(&self) -> impl Into<IndicesSource> {
        &self.ibo
    }
    fn get_program<'a>(&self) -> String {
        "fxaa".to_string()
    }
    fn get_draw_params(&self) -> DrawParameters {
        DrawParameters::default()
    }
}

pub struct FXAARenderer<'a> {
    framebuffer_color: &'a Texture2d,
    framebuffer: SimpleFrameBuffer<'a>,
}

impl<'a> FXAARenderer<'a> {
    pub fn new(ctx: &Context, fxaa: &'a FXAA) -> EngineResult<Self> {
        let framebuffer =
            SimpleFrameBuffer::with_depth_buffer(&ctx.display, &fxaa.color, &fxaa.depth)?;
        Ok(Self {
            framebuffer_color: &fxaa.color,
            framebuffer,
        })
    }

    pub fn draw_frame_buffer(
        &mut self,
        ctx: &mut Context,
        fxaa: &FXAA,
        renderer: &mut impl Renderer,
    ) -> EngineResult {
        let renderer_dimensions = renderer.get_dimensions();
        renderer.draw(
            ctx,
            fxaa,
            &uniform! {
                tex: self.framebuffer_color,
                resolution: (renderer_dimensions.0 as f32, renderer_dimensions.1 as f32)
            },
        )?;
        Ok(())
    }
}

impl Renderer for FXAARenderer<'_> {
    fn get_surface(&self) -> &impl Surface {
        &self.framebuffer
    }
    fn get_surface_mut(&mut self) -> &mut impl Surface {
        &mut self.framebuffer
    }
    fn draw<D, U>(&mut self, ctx: &mut Context, drawable: &D, uniforms: &U) -> EngineResult
    where
        D: Drawable,
        U: Uniforms,
    {
        let program = if let Some(program) = ctx.get_program(drawable.get_program()) {
            program
        } else {
            return Err(EngineError::Error(format!(
                "Program {} not found in the context",
                drawable.get_program()
            )));
        };

        self.framebuffer.draw(
            drawable.get_vbo(),
            drawable.get_ibo(),
            program,
            uniforms,
            &drawable.get_draw_params(),
        )?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    i_tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, i_tex_coords);
