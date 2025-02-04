use crate::{
    context::Context,
    error::EngineResult,
    renderer::{Drawable, Renderer},
};
use glium::{
    framebuffer::{DepthRenderBuffer, SimpleFrameBuffer},
    implement_vertex,
    index::{IndicesSource, PrimitiveType},
    program,
    texture::DepthFormat,
    uniform,
    uniforms::Uniforms,
    vertex::MultiVerticesSource,
    DrawParameters, IndexBuffer, Surface, Texture2d, VertexBuffer,
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

    pub fn send_program(ctx: &mut Context) -> EngineResult {
        ctx.add_program(
            "fxaa",
            program!(&ctx.display,
                100 => {
                    vertex: r"
                    #version 100

                    attribute vec2 position;
                    attribute vec2 i_tex_coords;

                    varying vec2 v_tex_coords;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                        v_tex_coords = i_tex_coords;
                    }
                ",
                    fragment: r"
                    #version 100

                    precision mediump float;

                    uniform vec2 resolution;
                    uniform sampler2D tex;

                    varying vec2 v_tex_coords;

                    #define FXAA_REDUCE_MIN   (1.0/ 128.0)
                    #define FXAA_REDUCE_MUL   (1.0 / 8.0)
                    #define FXAA_SPAN_MAX     8.0

                    vec4 fxaa(sampler2D tex, vec2 fragCoord, vec2 resolution,
                                vec2 v_rgbNW, vec2 v_rgbNE,
                                vec2 v_rgbSW, vec2 v_rgbSE,
                                vec2 v_rgbM) {
                        vec4 color;
                        mediump vec2 inverseVP = vec2(1.0 / resolution.x, 1.0 / resolution.y);
                        vec3 rgbNW = texture2D(tex, v_rgbNW).xyz;
                        vec3 rgbNE = texture2D(tex, v_rgbNE).xyz;
                        vec3 rgbSW = texture2D(tex, v_rgbSW).xyz;
                        vec3 rgbSE = texture2D(tex, v_rgbSE).xyz;
                        vec4 texColor = texture2D(tex, v_rgbM);
                        vec3 rgbM  = texColor.xyz;
                        vec3 luma = vec3(0.299, 0.587, 0.114);
                        float lumaNW = dot(rgbNW, luma);
                        float lumaNE = dot(rgbNE, luma);
                        float lumaSW = dot(rgbSW, luma);
                        float lumaSE = dot(rgbSE, luma);
                        float lumaM  = dot(rgbM,  luma);
                        float lumaMin = min(lumaM, min(min(lumaNW, lumaNE), min(lumaSW, lumaSE)));
                        float lumaMax = max(lumaM, max(max(lumaNW, lumaNE), max(lumaSW, lumaSE)));

                        mediump vec2 dir;
                        dir.x = -((lumaNW + lumaNE) - (lumaSW + lumaSE));
                        dir.y =  ((lumaNW + lumaSW) - (lumaNE + lumaSE));

                        float dirReduce = max((lumaNW + lumaNE + lumaSW + lumaSE) *
                                                (0.25 * FXAA_REDUCE_MUL), FXAA_REDUCE_MIN);

                        float rcpDirMin = 1.0 / (min(abs(dir.x), abs(dir.y)) + dirReduce);
                        dir = min(vec2(FXAA_SPAN_MAX, FXAA_SPAN_MAX),
                                    max(vec2(-FXAA_SPAN_MAX, -FXAA_SPAN_MAX),
                                    dir * rcpDirMin)) * inverseVP;

                        vec3 rgbA = 0.5 * (
                            texture2D(tex, fragCoord * inverseVP + dir * (1.0 / 3.0 - 0.5)).xyz +
                            texture2D(tex, fragCoord * inverseVP + dir * (2.0 / 3.0 - 0.5)).xyz);
                        vec3 rgbB = rgbA * 0.5 + 0.25 * (
                            texture2D(tex, fragCoord * inverseVP + dir * -0.5).xyz +
                            texture2D(tex, fragCoord * inverseVP + dir * 0.5).xyz);

                        float lumaB = dot(rgbB, luma);
                        if ((lumaB < lumaMin) || (lumaB > lumaMax))
                            color = vec4(rgbA, texColor.a);
                        else
                            color = vec4(rgbB, texColor.a);
                        return color;
                    }

                    void main() {
                        vec2 fragCoord = v_tex_coords * resolution;
                        vec4 color;
                            vec2 inverseVP = 1.0 / resolution.xy;
                            mediump vec2 v_rgbNW = (fragCoord + vec2(-1.0, -1.0)) * inverseVP;
                            mediump vec2 v_rgbNE = (fragCoord + vec2(1.0, -1.0)) * inverseVP;
                            mediump vec2 v_rgbSW = (fragCoord + vec2(-1.0, 1.0)) * inverseVP;
                            mediump vec2 v_rgbSE = (fragCoord + vec2(1.0, 1.0)) * inverseVP;
                            mediump vec2 v_rgbM = vec2(fragCoord * inverseVP);
                            color = fxaa(tex, fragCoord, resolution, v_rgbNW, v_rgbNE, v_rgbSW,
                                            v_rgbSE, v_rgbM);
                        gl_FragColor = color;
                    }
                ",
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

impl<'a> Renderer for FXAARenderer<'a> {
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
        self.framebuffer.draw(
            drawable.get_vbo(),
            drawable.get_ibo(),
            ctx.get_program(drawable.get_program()).expect(&format!(
                "Add program {} to the context",
                drawable.get_program()
            )),
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
