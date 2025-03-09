use bugsyth_engine::prelude::*;

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("gltf", (960, 720))?;
    ctx.new_program(
        "texture",
        "
    #version 140

    in vec3 position;
    in vec3 normal;
    in vec2 tex_coords;
    in vec4 color;

    out vec3 v_position;
    out vec3 v_normal;
    out vec2 v_tex_coords;
    out vec4 v_color;

    uniform mat4 persp;
    uniform mat4 view;

    void main() {
        v_position = position;
        v_normal = normal;
        v_tex_coords = tex_coords;
        v_color = color;
        gl_Position = persp * view * vec4(position, 1.0);
    }
    ",
        "
    #version 140

    const int max_lights = 100;

    in vec3 v_position;
    in vec3 v_normal;
    in vec2 v_tex_coords;
    in vec4 v_color;

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

        color = vec4(v_color.xyz * texture(u_tex, v_tex_coords).xyz + diffuse * diffuse_color + specular * specular_color, 1.0);
    }
    ",
        None,
    )
    .unwrap();
    let models = asset::load_gltf(&ctx.display, "resources/lil_man/lil_man.glb")?;
    let mut objs = Vec::new();
    for model in models {
        objs.push(Obj {
            model,
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
        });
    }
    let game = Game {
        objs,
        tex: Texture::new(&ctx, "resources/lil_man/lil_man.png")?,
    };
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    objs: Vec<Obj<'static>>,
    tex: Texture,
}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        bugsyth_engine::context::camera::CameraState::free_cam(ctx.dt, ctx, 1.0, 1.0);
    }
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        for obj in &self.objs {
            renderer
                .draw(
                    ctx,
                    obj,
                    &uniform! {
                        persp: ctx.camera.get_perspective(),
                        view: ctx.camera.get_view(),
                        u_light: [3.0, 10.0, 4.0f32],
                        u_tex: self.tex.get_texture_no_filtering(),
                    },
                )
                .unwrap();
        }
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
        self.ibo
    }
    fn get_program(&self) -> String {
        "texture".to_string()
    }
    fn get_draw_params(&self) -> DrawParameters {
        self.draw_params.clone()
    }
}
