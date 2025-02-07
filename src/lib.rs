/*
    game_wrapper will hold all the code that runs the game
*/

use context::Context;
use error::EngineResult;
use game_state::{app_wrapper::AppWrapper, GameState};
use glium::{backend::glutin::SimpleWindowBuilder, winit::event_loop::EventLoop};

pub mod context;
pub mod error;
pub mod game_state;
pub mod math;
pub mod prelude;
pub mod renderer;

pub mod glium {
    pub use glium::*;
}
pub mod obj;
pub mod rng;

pub fn init(
    window_title: impl Into<String>,
    window_size: (u32, u32),
) -> EngineResult<(EventLoop<()>, Context)> {
    let event_loop = EventLoop::new()?;
    let (window, display) = SimpleWindowBuilder::new()
        .with_title(&window_title.into())
        .with_inner_size(window_size.0, window_size.1)
        .build(&event_loop);
    Ok((event_loop, Context::new(window, display)?))
}

pub fn run(game: impl GameState, event_loop: EventLoop<()>, ctx: Context) -> EngineResult {
    let mut app = AppWrapper::new(game, ctx);
    event_loop.run_app(&mut app)?;
    Ok(())
}
