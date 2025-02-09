use bugsyth_engine::{context::audio::sound::Sound, prelude::*};

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("Audio", (960, 720))?;
    let game = Game {
        sound: Sound::new("resources/test.wav")?,
        t: 0.0,
    };
    for name in ctx.audio.get_output_device_names() {
        println!("{}", name);
    }
    ctx.audio.set_output_device_as_default_device()?;
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    sound: Sound,
    t: f32,
}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        self.t += ctx.dt;
        if self.t >= 1.0 {
            ctx.audio.play(&self.sound).unwrap();
            self.t -= 1.0;
        }
    }
    fn draw(&mut self, _ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color(0.0, 0.0, 0.0, 1.0);
    }
}
