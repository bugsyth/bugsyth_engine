use bugsyth_engine::prelude::*;

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("texture", (960, 720))?;
    ctx.new_program(
        "texture",
        "
    #version 140

    in vec3 position;
    in vec3 normal;
    in vec2 tex_coords;

    out vec3 v_position;
    out vec3 v_normal;
    out vec2 v_tex_coords;

    uniform mat4 persp;
    uniform mat4 view;

    void main() {
        v_position = position;
        v_normal = normal;
        v_tex_coords = tex_coords;
        gl_Position = persp * view * vec4(position, 1.0);
    }
    ",
        "
    #version 140

    const int max_lights = 100;

    in vec3 v_position;
    in vec3 v_normal;
    in vec2 v_tex_coords;

    out vec4 color;

    uniform vec3 u_light;
    uniform sampler2D u_tex;

    const vec3 diffuse_color = vec3(0.2, 0.2, 0.2);
    const vec3 specular_color = vec3(0.3, 0.3, 0.3);

    void main() {
        float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

        color = vec4(texture(u_tex, v_tex_coords).xyz + diffuse * diffuse_color + specular * specular_color, 1.0);
    }
    ",
        None,
    )
    .unwrap();
    let game = Game {
        obj: Obj {
            vbo: obj::load_wavefront(&ctx.display, &std::fs::read("resources/land.obj").unwrap())?,
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
        texture: Texture::new(&ctx, "resources/texture.png")?,
    };
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    obj: Obj<'static>,
    texture: Texture,
}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        bugsyth_engine::context::camera::CameraState::free_cam(ctx.dt, ctx, 1.0, 1.0);
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
                    u_light: [3.0, 10.0, 4.0f32],
                    u_tex: self.texture.get_texture()
                },
            )
            .unwrap();
    }
}

struct Obj<'a> {
    vbo: VertexBufferAny,
    ibo: NoIndices,
    draw_params: DrawParameters<'a>,
}

impl<'a> Drawable for Obj<'a> {
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
