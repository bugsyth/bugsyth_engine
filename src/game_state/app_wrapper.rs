use crate::{context::Context, game_state::GameState, renderer::FrameWrapper};
use glium::winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::WindowId,
};

use super::dt::DeltaTime;

pub struct AppWrapper<G> {
    pub ctx: Context,
    pub game: G,
    dt: DeltaTime,
}

impl<G> AppWrapper<G> {
    pub fn new(game: G, ctx: Context) -> Self {
        Self {
            ctx,
            game,
            dt: DeltaTime::new(),
        }
    }
}

impl<G> ApplicationHandler for AppWrapper<G>
where
    G: GameState,
{
    fn resumed(&mut self, _: &ActiveEventLoop) {}
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        self.ctx.input.process_input(&event);
        self.game.event(&mut self.ctx, &event);
        match event {
            WindowEvent::RedrawRequested => {
                let dt = self.dt.get_dt();
                self.ctx.dt = dt;
                self.game.update(&mut self.ctx);
                self.ctx.fixed_update.accumulator += dt;
                while self.ctx.fixed_update.accumulator > self.ctx.fixed_update.tick_rate {
                    self.game.fixed_update(&mut self.ctx);
                    self.ctx.fixed_update.accumulator -= self.ctx.fixed_update.tick_rate;
                }
                self.ctx.camera.update();
                let mut frame = FrameWrapper::new(self.ctx.display.draw());
                self.game.draw(&mut self.ctx, &mut frame);
                frame.finish();
                self.ctx.input.reset();
                self.ctx.window.request_redraw();
            }
            WindowEvent::Resized(new_size) => {
                self.ctx.display.resize(new_size.into());
            }
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            _ => {}
        }
    }
}
