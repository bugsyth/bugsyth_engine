use bugsyth_engine::prelude::*;
use glium::implement_vertex;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

fn main() -> EngineResult {
    let (event_loop, ctx) = init("Audio", (960, 720))?;
    let game = Game {};
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        ctx.audio.noise_test();
    }
    fn draw(&mut self, _ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color(0.0, 0.0, 0.0, 1.0);
    }
}
