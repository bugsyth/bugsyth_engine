// Also an example of displacement mapping

use bugsyth_engine::prelude::*;
use glium::{
    implement_vertex,
    index::{IndicesSource, NoIndices, PrimitiveType},
    uniform,
    vertex::MultiVerticesSource,
    winit::event::WindowEvent,
    BackfaceCullingMode, Depth, DepthTest, DrawParameters, VertexBuffer,
};

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("scolling texture", (960, 720))?;
    ctx.new_program(
        "texture",
        "
    #version 140

    in vec3 position;
    in vec2 tex_coords;

    out vec2 v_tex_coords;

    uniform mat4 persp;
    uniform mat4 view;

    void main() {
        v_tex_coords = tex_coords;
        gl_Position = persp * view * vec4(position, 1.0);
    }
    ",
        // It's the displacement map that is scolling not the texture
        // It makes a decent water effect
        "
    #version 140

    in vec2 v_tex_coords;

    out vec4 color;

    uniform vec2 u_direction;
    uniform sampler2D u_tex;
    uniform sampler2D u_displacement;
    uniform float u_time;

    void main() {
        vec2 displace = texture(u_displacement, v_tex_coords + u_direction * u_time * 0.1).xy;
        vec2 tex_coords = v_tex_coords + displace;
        color = texture(u_tex, tex_coords);
    }
    ",
        None,
    )
    .unwrap();
    let game = Game {
        imgui: bugsyth_engine_imgui_support::init(&ctx.window, &ctx.display, |_, _, _| {}),
        obj: Plane {
            vbo: VertexBuffer::new(
                &ctx.display,
                &[
                    // First triangle
                    Vertex {
                        position: [0.0, 0.0, 0.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex {
                        position: [1.0, 0.0, 0.0],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 0.0, 1.0],
                        tex_coords: [0.0, 1.0],
                    },
                    // Second triangle
                    Vertex {
                        position: [1.0, 0.0, 1.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex {
                        position: [0.0, 0.0, 1.0],
                        tex_coords: [0.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, 0.0, 0.0],
                        tex_coords: [1.0, 0.0],
                    },
                ],
            )
            .unwrap(),
            ibo: NoIndices(PrimitiveType::TrianglesList),
            draw_params: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullCounterClockwise,
                ..Default::default()
            },
        },
        texture: Texture::new(&ctx, "resources/water.png")?,
        displacement: Texture::new(&ctx, "resources/displacement.png")?,
        time: 0.0,
        direction: [0.0, 0.0],
    };
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    imgui: bugsyth_engine_imgui_support::ImGui,
    obj: Plane<'static>,
    texture: Texture,
    displacement: Texture,
    time: f32,
    direction: [f32; 2],
}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        self.imgui.update_dt(ctx.dt);
        bugsyth_engine::context::camera::CameraState::free_cam(ctx.dt, ctx, 1.0, 1.0);
        self.time += ctx.dt;
    }
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        renderer
            .draw(
                ctx,
                &self.obj,
                &uniform! {
                    persp: ctx.camera.get_perspective(),
                    view: ctx.camera.get_view(),
                    u_tex: self.texture.get_texture_no_filtering(),
                    // Turn off filtering to have a sharp displacement map
                    u_displacement: self.displacement.get_texture_no_filtering(),
                    u_time: self.time,
                    u_direction: self.direction,
                },
            )
            .unwrap();

        self.imgui
            .platform
            .prepare_frame(self.imgui.context.io_mut(), &ctx.window)
            .unwrap();
        let ui = self.imgui.context.frame();

        ui.window("imgui")
            .size(
                [250.0, 150.0],
                bugsyth_engine_imgui_support::Condition::FirstUseEver,
            )
            .build(|| {
                ui.text_wrapped("Direction:");

                ui.slider("x", -1.0, 1.0, &mut self.direction[0]);
                ui.slider("y", -1.0, 1.0, &mut self.direction[1]);

                ui.separator();
                ui.text(format!("FPS: {}", (1.0 / ctx.dt).floor()));
            });

        self.imgui.platform.prepare_render(&ui, &ctx.window);
        let draw_data = self.imgui.context.render();
        self.imgui
            .renderer
            .render(renderer.get_surface_mut(), draw_data)
            .unwrap();
    }
    fn event(&mut self, ctx: &mut Context, event: &WindowEvent) {
        self.imgui.event(&ctx.window, event);
    }
}

struct Plane<'a> {
    vbo: VertexBuffer<Vertex>,
    ibo: NoIndices,
    draw_params: DrawParameters<'a>,
}

impl<'a> Drawable for Plane<'a> {
    fn get_vbo(&self) -> impl MultiVerticesSource {
        &self.vbo
    }
    fn get_ibo(&self) -> impl Into<IndicesSource> {
        &self.ibo
    }
    fn get_program(&self) -> String {
        "texture".to_string()
    }
    fn get_draw_params(&self) -> DrawParameters {
        self.draw_params.clone()
    }
}

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);
