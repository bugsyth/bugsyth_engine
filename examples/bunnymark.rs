use std::time::{Duration, Instant};

use bugsyth_engine::prelude::*;
use glium::{IndexBuffer, Surface};
use rand::Rng;

const INITIAL_BUNNIES: usize = 100;
const MAX_X: f32 = 1.0;
const MAX_Y: f32 = 1.0;
const GRAVITY: f32 = 0.01;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
}
implement_vertex!(Vertex, position, uv);
#[derive(Clone, Copy)]
struct Instance {
    instance: [f32; 2],
}
implement_vertex!(Instance, instance);

#[derive(Clone, Copy)]
struct Bunny {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Bunny {
    fn new(rng: &mut rand::rngs::ThreadRng) -> Self {
        let x_vel = rng.random::<f32>() * 0.1;
        let y_vel = rng.random::<f32>() * 0.1;
        Self {
            position: Vec2::zero(),
            velocity: Vec2::new(x_vel, y_vel),
        }
    }
}

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("Bunnymark", (960, 720))?;
    ctx.new_program(
        "bunnymark",
        "
    #version 140
    in vec2 position;
    in vec2 uv;
    in vec2 instance;

    out vec2 v_uv;

    void main() {
        v_uv = uv;
        gl_Position = vec4(position + instance, 0.0, 1.0);
    }
    ",
        "
    #version 140
    in vec2 v_uv;

    uniform sampler2D tex;

    out vec4 color;

    void main() {
        vec4 tex_color = texture(tex, v_uv);
        color = tex_color;
    }
    ",
        None,
    )
    .unwrap();

    let mut rng = rand::rng();
    let bunnies: Vec<Bunny> = (0..INITIAL_BUNNIES).map(|_| Bunny::new(&mut rng)).collect();
    let vbo = VertexBuffer::new(
        &ctx.display,
        &[
            Vertex {
                position: [0.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.1, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.1, 0.19],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [0.0, 0.19],
                uv: [0.0, 1.0],
            },
        ],
    )
    .unwrap();
    let ibo = IndexBuffer::new(
        &ctx.display,
        PrimitiveType::TrianglesList,
        &[0, 1, 2, 0, 3, 2],
    )
    .unwrap();

    let ivbo_data: Vec<Instance> = (0..bunnies.len())
        .map(|_| Instance {
            instance: [0.0, 0.0],
        })
        .collect();
    let game = Game {
        bunnies,
        tex: Texture::new(&ctx, "resources/wabbit_alpha.png").unwrap(),
        vbo,
        ivbo: VertexBuffer::dynamic(&ctx.display, &ivbo_data).unwrap(),
        ibo,
        params: DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        },

        rng,
    };
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    bunnies: Vec<Bunny>,
    tex: Texture,
    vbo: VertexBuffer<Vertex>,
    ivbo: VertexBuffer<Instance>,
    ibo: IndexBuffer<u16>,
    params: DrawParameters<'static>,

    rng: rand::rngs::ThreadRng,
}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        let random_y_boost: Vec<f32> = (0..self.bunnies.len())
            .map(|_| self.rng.random::<f32>() * 0.075)
            .collect();
        if ctx.input.is_key_pressed(KeyCode::Space) {
            for _ in 0..INITIAL_BUNNIES {
                self.bunnies.push(Bunny::new(&mut self.rng));
            }
        }

        let mut ivbo_data = Vec::new();
        for (i, bunny) in self.bunnies.iter_mut().enumerate() {
            bunny.position += bunny.velocity;
            bunny.velocity.y -= GRAVITY;

            if bunny.position.x > MAX_X {
                bunny.velocity.x *= -1.0;
                bunny.position.x = MAX_X;
            } else if bunny.position.x < -MAX_X {
                bunny.velocity.x *= -1.0;
                bunny.position.x = -MAX_X;
            }

            if bunny.position.y > MAX_Y {
                bunny.velocity.y = 0.0;
                bunny.position.y = MAX_Y;
            } else if bunny.position.y < -MAX_Y {
                bunny.velocity.y *= -0.8;
                bunny.position.y = -MAX_Y;

                if self.rng.random::<bool>() {
                    bunny.velocity.y += random_y_boost[i];
                }
            }
            ivbo_data.push(Instance {
                instance: [bunny.position.x, bunny.position.y],
            });
        }
        if ivbo_data.len() > self.ivbo.len() {
            self.ivbo = VertexBuffer::dynamic(&ctx.display, &ivbo_data).unwrap();
        } else {
            self.ivbo.write(&ivbo_data);
        }
        self.ivbo.write(&ivbo_data);
        ctx.window.set_title(&format!(
            "Drawing {} bunnies at {:.0} fps",
            self.bunnies.len(),
            1.0 / ctx.dt
        ));
    }
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color(0.0, 0.0, 0.0, 1.0);
        let target = renderer.get_surface_mut();
        let tex = self.tex.get_texture_no_filtering();

        target
            .draw(
                (&self.vbo, self.ivbo.per_instance().unwrap()),
                &self.ibo,
                ctx.get_program("bunnymark").unwrap(),
                &uniform! {
                    tex: tex,
                },
                &self.params,
            )
            .unwrap();
    }
}
