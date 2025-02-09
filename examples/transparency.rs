use bugsyth_engine::prelude::*;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}
implement_vertex!(Vertex, position, color);

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("transparency", (960, 720))?;
    ctx.new_program(
        "3d",
        "
    in vec3 position;
    in vec4 color;

    out vec4 v_color;

    uniform mat4 persp;
    uniform mat4 view;

    void main() {
        v_color = color;
        gl_Position = persp * view * vec4(position, 1.0);
    }
    ",
        "
    in vec4 v_color;

    out vec4 color;

    void main() {
        color = v_color;
    }
    ",
        None,
    )
    .unwrap();
    let imgui = bugsyth_engine_imgui_support::init(&ctx.window, &ctx.display, |_, _, _| {});

    let tri_positions = [[-0.5, -0.5, 0.0], [0.5, 0.5, 0.0], [-0.5, 0.5, 0.0]];
    let tri_colors = [
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
    ];
    let transparent_tri_positions = [[-0.5, -0.5, 1.0], [0.5, 0.5, 1.0], [-0.5, 0.5, 1.0]];
    let transparent_tri_colors = [
        [1.0, 0.0, 0.0, 0.5],
        [0.0, 1.0, 0.0, 0.5],
        [0.0, 0.0, 1.0, 0.5],
    ];
    let draw_params = DrawParameters {
        depth: Depth {
            test: DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        blend: Blend::alpha_blending(),
        ..Default::default()
    };
    let game = Game {
        tri: Triangle {
            vbo: VertexBuffer::new(
                &ctx.display,
                &[
                    Vertex {
                        position: tri_positions[0],
                        color: tri_colors[0],
                    },
                    Vertex {
                        position: tri_positions[1],
                        color: tri_colors[1],
                    },
                    Vertex {
                        position: tri_positions[2],
                        color: tri_colors[2],
                    },
                ],
            )
            .unwrap(),
            positions: tri_positions,
            colors: tri_colors,
            ibo: NoIndices(PrimitiveType::TrianglesList),
            draw_params: draw_params.clone(),
        },
        tansparent_tri: Triangle {
            vbo: VertexBuffer::new(
                &ctx.display,
                &[
                    Vertex {
                        position: transparent_tri_positions[0],
                        color: transparent_tri_colors[0],
                    },
                    Vertex {
                        position: transparent_tri_positions[1],
                        color: transparent_tri_colors[1],
                    },
                    Vertex {
                        position: transparent_tri_positions[2],
                        color: transparent_tri_colors[2],
                    },
                ],
            )
            .unwrap(),
            positions: transparent_tri_positions,
            colors: transparent_tri_colors,
            ibo: NoIndices(PrimitiveType::TrianglesList),
            draw_params: draw_params,
        },
        alpha: 0.5,
        imgui,
    };
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    tri: Triangle<'static>,
    tansparent_tri: Triangle<'static>,
    alpha: f32,
    imgui: bugsyth_engine_imgui_support::ImGui,
}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        self.imgui.update_dt(ctx.dt);
        bugsyth_engine::context::camera::CameraState::free_cam(ctx.dt, ctx, 1.0, 1.0);
    }
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
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
                ui.slider("Transparency", 0.0, 1.0, &mut self.alpha);

                ui.separator();
                ui.text(format!("FPS: {}", (1.0 / ctx.dt).floor()));
            });

        for color in &mut self.tansparent_tri.colors {
            color[3] = self.alpha;
        }

        self.tansparent_tri.vbo = VertexBuffer::new(
            &ctx.display,
            &[
                Vertex {
                    position: self.tansparent_tri.positions[0],
                    color: self.tansparent_tri.colors[0],
                },
                Vertex {
                    position: self.tansparent_tri.positions[1],
                    color: self.tansparent_tri.colors[1],
                },
                Vertex {
                    position: self.tansparent_tri.positions[2],
                    color: self.tansparent_tri.colors[2],
                },
            ],
        )
        .unwrap();

        /*
            Must draw the opaque objects first, then
            draw the transparent objects in order of
            farthest to closest to the view
        */
        renderer
            .draw(
                ctx,
                &self.tri,
                &uniform! {
                    persp: ctx.camera.get_perspective(),
                    view: ctx.camera.get_view(),
                },
            )
            .unwrap();
        renderer
            .draw(
                ctx,
                &self.tansparent_tri,
                &uniform! {
                    persp: ctx.camera.get_perspective(),
                    view: ctx.camera.get_view(),
                },
            )
            .unwrap();

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

struct Triangle<'a> {
    vbo: VertexBuffer<Vertex>,
    positions: [[f32; 3]; 3],
    colors: [[f32; 4]; 3],
    ibo: NoIndices,
    draw_params: DrawParameters<'a>,
}

impl<'a> Drawable for Triangle<'a> {
    fn get_vbo(&self) -> impl MultiVerticesSource {
        &self.vbo
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
