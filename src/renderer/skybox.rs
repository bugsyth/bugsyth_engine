// Something with the ordering in the example doesnt work
// Fix it sometime?

// I think it is fixed
// Images are seen as if the texture was on the outside so thing like text are backwards

use crate::{
    context::Context,
    error::EngineResult,
    renderer::{Drawable, texture::Texture},
    shaders::{SKYBOX_FS, SKYBOX_VS},
};
use glium::{
    BackfaceCullingMode, BlitTarget, Depth, DepthTest, DrawParameters, IndexBuffer, Surface,
    VertexBuffer,
    framebuffer::SimpleFrameBuffer,
    implement_vertex,
    index::{IndicesSource, PrimitiveType},
    program,
    texture::{self, Cubemap},
    uniforms::{MagnifySamplerFilter, Sampler},
    vertex::MultiVerticesSource,
};
use std::path::Path;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

/// Skybox program must be sent to the Context before rendering
pub struct Skybox {
    cubemap: texture::Cubemap,
    resolution: u32,
    right: Texture,
    left: Texture,
    top: Texture,
    bottom: Texture,
    front: Texture,
    back: Texture,
    vbo: VertexBuffer<Vertex>,
    ibo: IndexBuffer<u16>,
}

impl Skybox {
    /// resolution is the size of your textures
    /// paths: [right, left, top, bottom, front, back]
    pub fn new(
        ctx: &Context,
        resolution: u32,
        paths: [impl AsRef<Path> + Clone; 6],
        // right: impl AsRef<Path>,
        // left: impl AsRef<Path>,
        // top: impl AsRef<Path>,
        // bottom: impl AsRef<Path>,
        // front: impl AsRef<Path>,
        // back: impl AsRef<Path>,
    ) -> EngineResult<Self> {
        let scale = 25.0;
        Ok(Self {
            cubemap: texture::Cubemap::empty(&ctx.display, resolution)?,
            resolution,
            right: Texture::new(ctx, paths[0].clone())?,
            left: Texture::new(ctx, paths[1].clone())?,
            top: Texture::new(ctx, paths[2].clone())?,
            bottom: Texture::new(ctx, paths[3].clone())?,
            front: Texture::new(ctx, paths[4].clone())?,
            back: Texture::new(ctx, paths[5].clone())?,
            vbo: VertexBuffer::new(
                &ctx.display,
                &[
                    // Front
                    Vertex {
                        position: [-scale, -scale, scale],
                    },
                    Vertex {
                        position: [scale, -scale, scale],
                    },
                    Vertex {
                        position: [scale, scale, scale],
                    },
                    Vertex {
                        position: [-scale, scale, scale],
                    },
                    // Right
                    Vertex {
                        position: [scale, -scale, scale],
                    },
                    Vertex {
                        position: [scale, -scale, -scale],
                    },
                    Vertex {
                        position: [scale, scale, -scale],
                    },
                    Vertex {
                        position: [scale, scale, scale],
                    },
                    // Back
                    Vertex {
                        position: [-scale, -scale, -scale],
                    },
                    Vertex {
                        position: [-scale, scale, -scale],
                    },
                    Vertex {
                        position: [scale, scale, -scale],
                    },
                    Vertex {
                        position: [scale, -scale, -scale],
                    },
                    // Left
                    Vertex {
                        position: [-scale, -scale, scale],
                    },
                    Vertex {
                        position: [-scale, scale, scale],
                    },
                    Vertex {
                        position: [-scale, scale, -scale],
                    },
                    Vertex {
                        position: [-scale, -scale, -scale],
                    },
                    // Bottom
                    Vertex {
                        position: [-scale, -scale, scale],
                    },
                    Vertex {
                        position: [-scale, -scale, -scale],
                    },
                    Vertex {
                        position: [scale, -scale, -scale],
                    },
                    Vertex {
                        position: [scale, -scale, scale],
                    },
                    // Top
                    Vertex {
                        position: [-scale, scale, scale],
                    },
                    Vertex {
                        position: [scale, scale, scale],
                    },
                    Vertex {
                        position: [scale, scale, -scale],
                    },
                    Vertex {
                        position: [-scale, scale, -scale],
                    },
                ],
            )
            .unwrap(),
            ibo: IndexBuffer::new(
                &ctx.display,
                PrimitiveType::TrianglesList,
                &[
                    // Front
                    0u16, 2, 1, 0, 3, 2, // Right
                    4, 6, 5, 4, 7, 6, // Back
                    8, 10, 9, 8, 11, 10, // Left
                    12, 14, 13, 12, 15, 14, // Bottom
                    16, 18, 17, 16, 19, 18, // Top
                    20, 22, 21, 20, 23, 22,
                ],
            )
            .unwrap(),
        })
    }

    /// Sends a program called 'skybox' to the Context
    pub fn send_program(ctx: &mut Context) -> EngineResult {
        ctx.add_program(
            "skybox",
            program!(&ctx.display, 140 => {
                vertex: SKYBOX_VS,
                fragment: SKYBOX_FS,
            })?,
        );
        Ok(())
    }

    pub fn get_cubemap(&self) -> Sampler<'_, Cubemap> {
        self.cubemap
            .sampled()
            .magnify_filter(MagnifySamplerFilter::Linear)
    }

    pub fn prepare_draw(
        &self,
        ctx: &mut Context,
        magnify_filter: MagnifySamplerFilter,
    ) -> EngineResult {
        let blit_target = BlitTarget {
            left: 0,
            bottom: 0,
            width: self.resolution as i32,
            height: self.resolution as i32,
        };

        let framebuffer1 = SimpleFrameBuffer::new(
            &ctx.display,
            self.cubemap
                .main_level()
                .image(glium::texture::CubeLayer::PositiveX),
        )?;
        let framebuffer2 = SimpleFrameBuffer::new(
            &ctx.display,
            self.cubemap
                .main_level()
                .image(glium::texture::CubeLayer::NegativeX),
        )?;
        let framebuffer3 = SimpleFrameBuffer::new(
            &ctx.display,
            self.cubemap
                .main_level()
                .image(glium::texture::CubeLayer::PositiveY),
        )?;
        let framebuffer4 = SimpleFrameBuffer::new(
            &ctx.display,
            self.cubemap
                .main_level()
                .image(glium::texture::CubeLayer::NegativeY),
        )?;
        let framebuffer5 = SimpleFrameBuffer::new(
            &ctx.display,
            self.cubemap
                .main_level()
                .image(glium::texture::CubeLayer::PositiveZ),
        )?;
        let framebuffer6 = SimpleFrameBuffer::new(
            &ctx.display,
            self.cubemap
                .main_level()
                .image(glium::texture::CubeLayer::NegativeZ),
        )?;

        self.right.get_texture().as_surface().blit_whole_color_to(
            &framebuffer1,
            &blit_target,
            magnify_filter,
        );
        self.left.get_texture().as_surface().blit_whole_color_to(
            &framebuffer2,
            &blit_target,
            magnify_filter,
        );
        self.bottom.get_texture().as_surface().blit_whole_color_to(
            &framebuffer3,
            &blit_target,
            magnify_filter,
        );
        self.top.get_texture().as_surface().blit_whole_color_to(
            &framebuffer4,
            &blit_target,
            magnify_filter,
        );
        self.front.get_texture().as_surface().blit_whole_color_to(
            &framebuffer5,
            &blit_target,
            magnify_filter,
        );
        self.back.get_texture().as_surface().blit_whole_color_to(
            &framebuffer6,
            &blit_target,
            magnify_filter,
        );
        Ok(())
    }
}

impl Drawable for Skybox {
    fn get_vbo(&self) -> impl MultiVerticesSource {
        &self.vbo
    }
    fn get_ibo(&self) -> impl Into<IndicesSource> {
        &self.ibo
    }
    fn get_program(&self) -> String {
        "skybox".to_string()
    }
    fn get_draw_params(&self) -> DrawParameters {
        DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: false,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        }
    }
}
