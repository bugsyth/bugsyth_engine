use std::fs;

use bugsyth_engine::prelude::*;
use glium::{Program, Surface};

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("text", (960, 720))?;
    ctx.add_font("dogica", &fs::read("resources/dogica.ttf").unwrap(), 72.0)?;
    let scale = 0.001;
    let wrapping = Some((256.0, 96.0));
    let str = "Hello, World! The quick brown fox jumps over the lazy dog.";
    let font_name = "dogica";

    // Doesnt create a text object, can be used to check bounds before creating the object
    let dim = Text::get_text_dimensions(&ctx, scale, wrapping, str, font_name).unwrap();
    println!("(w, h) - {:?}", dim);

    let text = Text::new(&ctx, Vec2::new(-0.3, 0.3), scale, wrapping, str, font_name)?;
    println!("(w, h) - {:?}", text.get_dimensions());

    let game = Game { text };
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    text: Text,
}

impl GameState for Game {
    fn update(&mut self, _ctx: &mut Context) {}
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color(0.0, 0.0, 0.0, 1.0);
        self.text.draw(ctx, renderer).unwrap();

        // Bounding box for the text, move to variables to Game
        {
            #[derive(Clone, Copy)]
            struct Vertex {
                pos: [f32; 2],
            }
            implement_vertex!(Vertex, pos);

            let (width, height) = self.text.get_dimensions();
            let vbo = VertexBuffer::new(
                &ctx.display,
                &[
                    Vertex {
                        pos: self.text.pos.into_array(),
                    },
                    Vertex {
                        pos: [self.text.pos.x + width, self.text.pos.y],
                    },
                    Vertex {
                        pos: [self.text.pos.x + width, self.text.pos.y - height],
                    },
                    Vertex {
                        pos: [self.text.pos.x, self.text.pos.y - height],
                    },
                ],
            )
            .unwrap();
            let ibo = IndexBuffer::new(&ctx.display, PrimitiveType::LineStrip, &[0, 1, 2, 3, 0u8])
                .unwrap();
            let program = Program::from_source(
                &ctx.display,
                &"in vec2 pos;
                                void main() {
                                    gl_Position = vec4(pos, 0.0, 1.0);
                                }",
                "
                                out vec4 color;
                                void main() {
                                color = vec4(1.0);
                                }",
                None,
            )
            .unwrap();
            renderer
                .get_surface_mut()
                .draw(
                    &vbo,
                    &ibo,
                    &program,
                    &uniform! {},
                    &DrawParameters::default(),
                )
                .unwrap();
        }
    }
}
