//! Small framework I plan to use for my own games
//!
//! To get a window on the screen, call (init)[https://docs.rs/bugsyth_engine/0.4.0/bugsyth_engine/fn.init.html]
//! If you want to draw stuff onto the screen, you need a shader,
//! something that implements the [Drawable](https://docs.rs/bugsyth_engine/0.4.0/bugsyth_engine/renderer/drawable/trait.Drawable.html) trait,
//! a struct that implements [GameState](https://docs.rs/bugsyth_engine/0.4.0/bugsyth_engine/game_state/trait.GameState.html) to hold your game's data,
//! and then finally to run it all by calling [run](https://docs.rs/bugsyth_engine/0.4.0/bugsyth_engine/fn.run.html)

/*
    game_wrapper will hold all the code that runs the game
    re-exports: vek, gltf, glium, fastrand
*/

use context::Context;
use error::EngineResult;
use game_state::{GameState, app_wrapper::AppWrapper};
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
pub mod asset;
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
    app.game.init(&mut app.ctx);
    event_loop.run_app(&mut app)?;
    Ok(())
}
