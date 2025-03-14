use bugsyth_engine::{
    prelude::*,
    renderer::fxaa::{FXAA, FXAARenderer},
};

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("fxaa", (960, 720))?;
    ctx.new_program(
        "lighting",
        "
    #version 140

    in vec3 position;
    in vec3 normal;
    in vec2 tex_coords;

    out vec3 v_position;
    out vec3 v_normal;

    uniform mat4 persp;
    uniform mat4 view;

    void main() {
        v_position = position;
        v_normal = normal;
        gl_Position = persp * view * vec4(position, 1.0);
    }
    ",
        "
    #version 140

    const int max_lights = 100;

    in vec3 v_position;
    in vec3 v_normal;

    out vec4 color;

    uniform vec3 u_light;

    const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
    const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
    const vec3 specular_color = vec3(1.0, 1.0, 1.0);

    void main() {
        float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

        color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
    }
    ",
        None,
    )
    .unwrap();
    FXAA::send_program(&mut ctx)?;
    let window_size = ctx.window.inner_size();
    let game = Game {
        obj: Obj {
            model: asset::load_wavefront(&ctx, &std::fs::read("resources/suzanne.obj").unwrap())?,
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
        // Divided by 4 for exaggerated effect
        fxaa: FXAA::new(&ctx, window_size.width / 4, window_size.height / 4)?,
    };
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    obj: Obj<'static>,
    fxaa: FXAA,
}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        bugsyth_engine::context::camera::CameraState::free_cam(ctx.dt, ctx, 1.0, 1.0);
    }
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color(0.0, 0.0, 0.0, 1.0);
        let mut fxaa_renderer = FXAARenderer::new(&ctx, &self.fxaa).unwrap();
        fxaa_renderer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        fxaa_renderer
            .draw(
                ctx,
                &self.obj,
                &uniform! {
                    persp: ctx.camera.get_perspective(),
                    view: ctx.camera.get_view(),
                    u_light: [100.0, 40.0, 50.0f32],
                },
            )
            .unwrap();
        fxaa_renderer
            .draw_frame_buffer(ctx, &self.fxaa, renderer)
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
        "lighting".to_string()
    }
    fn get_draw_params(&self) -> DrawParameters {
        self.draw_params.clone()
    }
}
