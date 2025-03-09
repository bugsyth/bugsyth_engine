// Doesn't render the skybox textures properly
// Will fix later

use bugsyth_engine::prelude::*;

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("skybox", (960, 720))?;
    ctx.new_program(
        "3d",
        "
    in vec3 position;
    in vec3 normal;
    in vec2 tex_coords;

    out vec3 v_normal;

    uniform mat4 persp;
    uniform mat4 view;

    void main() {
        v_normal = normal;
        gl_Position = persp * view * vec4(position, 1.0);
    }
    ",
        "
    in vec3 v_normal;

    out vec4 color;

    void main() {
        color = vec4(v_normal, 1.0);
    }
    ",
        None,
    )
    .unwrap();
    Skybox::send_program(&mut ctx)?;
    let game = Game {
        obj: Obj {
            model: asset::load_wavefront(
                &ctx.display,
                &std::fs::read("resources/land.obj").unwrap(),
            )?,
            ibo: NoIndices(PrimitiveType::TrianglesList),
            draw_params: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullClockwise,
                ..Default::default()
            },
        },
        skybox: Skybox::new(
            &ctx,
            128,
            [
                "resources/skybox/right.png",
                "resources/skybox/left.png",
                "resources/skybox/top.png",
                "resources/skybox/bottom.png",
                "resources/skybox/front.png",
                "resources/skybox/back.png",
            ],
        )?,
    };
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    obj: Obj<'static>,
    skybox: Skybox,
}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        bugsyth_engine::context::camera::CameraState::free_cam(ctx.dt, ctx, 1.0, 1.0);
    }
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        self.skybox
            .prepare_draw(ctx, glium::uniforms::MagnifySamplerFilter::Nearest)
            .unwrap();
        renderer
            .draw(
                ctx,
                &self.skybox,
                &uniform! {
                    persp: ctx.camera.get_perspective(),
                    view: ctx.camera.get_view(),
                    skybox: self.skybox.get_cubemap(),
                },
            )
            .unwrap();
        renderer
            .draw(
                ctx,
                &self.obj,
                &uniform! {
                    persp: ctx.camera.get_perspective(),
                    view: ctx.camera.get_view(),
                },
            )
            .unwrap();
    }
}

struct Obj<'a> {
    model: Model,
    ibo: NoIndices,
    draw_params: DrawParameters<'a>,
}

impl<'a> Drawable for Obj<'a> {
    fn get_vbo(&self) -> impl MultiVerticesSource {
        self.model.get_vbo()
    }
    fn get_ibo(&self) -> impl Into<IndicesSource> {
        &self.ibo
    }
    fn get_program(&self) -> String {
        "3d".to_string()
    }
    fn get_draw_params(&self) -> DrawParameters {
        self.draw_params.clone()
    }
}
