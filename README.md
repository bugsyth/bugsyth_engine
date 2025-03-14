More a framework, not an engine

Uses glium for rendering, winit for event handling, and cpal for audio

## Example

```rust ,noplaypen
use bugsyth_engine::prelude::*;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("Simple", (960, 720))?;
    ctx.new_program(
        "simple",
        "
    in vec2 position;
    in vec3 color;

    out vec3 v_color;

    void main() {
        v_color = color;
        gl_Position = vec4(position, 0.0, 1.0);
    }
    ",
        "
    in vec3 v_color;

    out vec4 color;

    void main() {
        color = vec4(v_color, 1.0);
    }
    ",
        None,
    )
    .unwrap();
    let game = Game {
        tri: Triangle {
            vbo: VertexBuffer::new(
                &ctx.display,
                &[
                    Vertex {
                        position: [-0.5, -0.5],
                        color: [1.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [0.5, 0.5],
                        color: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [-0.5, 0.5],
                        color: [0.0, 0.0, 1.0],
                    },
                ],
            )
            .unwrap(),
            ibo: NoIndices(PrimitiveType::TrianglesList),
            draw_params: DrawParameters {
                ..Default::default()
            },
        },
    };
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    tri: Triangle<'static>,
}

impl GameState for Game {
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer.draw(ctx, &self.tri, &uniform! {}).unwrap();
    }
}

struct Triangle<'a> {
    vbo: VertexBuffer<Vertex>,
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
        "simple".to_string()
    }
    fn get_draw_params(&self) -> DrawParameters {
        self.draw_params.clone()
    }
}
```

More examples [here](https://github.com/bugsyth/bugsyth_engine/tree/master/examples)