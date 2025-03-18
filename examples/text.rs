use std::fs;

use bugsyth_engine::prelude::*;

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("text", (960, 720))?;
    ctx.add_font("dogica", &fs::read("resources/dogica.ttf").unwrap(), 72.0)?;
    let scale = 0.001;
    let str = "Hello, World!   The quick brown fox jumps over the lazy dog.";
    let font_name = "dogica";

    // Doesnt create a text object, can be used to check bounds before creating the object
    let dim = Text::get_dimensions(&ctx, scale, str, font_name).unwrap();
    println!("(w, h) - {:?}", dim);

    let text = Text::new(
        &ctx,
        Vec2::new(-1.0, 1.0),
        scale,
        Some((1024.0, 96.0)),
        str,
        font_name,
    )?;
    println!("Text width = {}", text.width);
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
    }
}
