use bugsyth_engine::prelude::*;

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("Audio", (960, 720))?;
    let game = Game {
        imgui: bugsyth_engine_imgui_support::init(&ctx.window, &ctx.display, |_, _, _| {}),
        sound: Sound::new("resources/goron.wav")?,
        speed: audio_play_value::new_audio_play_value(1.0),
        volume: audio_play_value::new_audio_play_value(1.0),
        speed_float: 1.0,
        volume_float: 1.0,
    };
    for name in ctx.audio.get_output_device_names() {
        println!("{}", name);
    }
    ctx.audio.set_output_device_as_default_device()?;
    ctx.audio.play(&game.sound, &game.volume, &game.speed)?;
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    imgui: bugsyth_engine_imgui_support::ImGui,
    sound: Sound,
    speed: audio_play_value::AudioPlayValue,
    volume: audio_play_value::AudioPlayValue,
    speed_float: f64,
    volume_float: f64,
}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        self.imgui.update_dt(ctx.dt);
    }
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color(0.0, 0.0, 0.0, 1.0);
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
                ui.text_wrapped("Audio settings:");
                ui.slider("speed", 0.0, 2.0, &mut self.speed_float);
                ui.slider("volume", 0.0, 2.0, &mut self.volume_float);
            });

        self.imgui.platform.prepare_render(&ui, &ctx.window);
        let draw_data = self.imgui.context.render();
        self.imgui
            .renderer
            .render(renderer.get_surface_mut(), draw_data)
            .unwrap();
        audio_play_value::atomic_f64_store(&self.speed, self.speed_float);
        audio_play_value::atomic_f64_store(&self.volume, self.volume_float);
    }
    fn event(&mut self, ctx: &mut Context, event: &WindowEvent) {
        self.imgui.event(&ctx.window, event);
    }
}
