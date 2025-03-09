//! Small framework I plan to use for my own games
//!
//! ## Example usage
//!
//! To get a window on the screen, call (init)[https://docs.rs/bugsyth_engine/0.4.0/bugsyth_engine/fn.init.html]
//! If you want to draw stuff onto the screen, you need a shader,
//! something that implements the [Drawable](https://docs.rs/bugsyth_engine/0.4.0/bugsyth_engine/renderer/drawable/trait.Drawable.html) trait,
//! a struct that implements [GameState](https://docs.rs/bugsyth_engine/0.4.0/bugsyth_engine/game_state/trait.GameState.html) to hold your game's data,
//! and then finally to run it all by calling [run](https://docs.rs/bugsyth_engine/0.4.0/bugsyth_engine/fn.run.html)
//!
//! Here is an example of putting it together
//!
//! ```
//! use bugsyth_engine::prelude::*;
//!
//! #[derive(Clone, Copy)]
//! struct Vertex {
//!     position: [f32; 2],
//!     color: [f32; 3],
//! }
//! implement_vertex!(Vertex, position, color);
//!
//! fn main() -> EngineResult {
//!     let (event_loop, mut ctx) = init("Simple", (960, 720))?;
//!     ctx.new_program(
//!         "simple",
//!         "
//!     in vec2 position;
//!     in vec3 color;
//!
//!     out vec3 v_color;
//!
//!     void main() {
//!         v_color = color;
//!         gl_Position = vec4(position, 0.0, 1.0);
//!     }
//!     ",
//!         "
//!     in vec3 v_color;
//!
//!     out vec4 color;
//!
//!     void main() {
//!         color = vec4(v_color, 1.0);
//!     }
//!     ",
//!         None,
//!     )
//!     .unwrap();
//!     let game = Game {
//!         tri: Triangle {
//!             vbo: VertexBuffer::new(
//!                 &ctx.display,
//!                 &[
//!                     Vertex {
//!                         position: [-0.5, -0.5],
//!                         color: [1.0, 0.0, 0.0],
//!                     },
//!                     Vertex {
//!                         position: [0.5, 0.5],
//!                         color: [0.0, 1.0, 0.0],
//!                     },
//!                     Vertex {
//!                         position: [-0.5, 0.5],
//!                         color: [0.0, 0.0, 1.0],
//!                     },
//!                 ],
//!             )
//!             .unwrap(),
//!             ibo: NoIndices(PrimitiveType::TrianglesList),
//!             draw_params: DrawParameters {
//!                 ..Default::default()
//!             },
//!         },
//!     };
//!     run(game, event_loop, ctx)?;
//!     Ok(())
//! }
//!
//! struct Game {
//!     tri: Triangle<'static>,
//! }
//!
//! impl GameState for Game {
//!     fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
//!         renderer.clear_color(0.0, 0.0, 0.0, 1.0);
//!         renderer.draw(ctx, &self.tri, &uniform! {}).unwrap();
//!     }
//! }
//!
//! struct Triangle<'a> {
//!     vbo: VertexBuffer<Vertex>,
//!     ibo: NoIndices,
//!     draw_params: DrawParameters<'a>,
//! }
//!
//! impl<'a> Drawable for Triangle<'a> {
//!     fn get_vbo(&self) -> impl MultiVerticesSource {
//!         &self.vbo
//!     }
//!     fn get_ibo(&self) -> impl Into<IndicesSource> {
//!         &self.ibo
//!     }
//!     fn get_program(&self) -> String {
//!         "simple".to_string()
//!     }
//!     fn get_draw_params(&self) -> DrawParameters {
//!         self.draw_params.clone()
//!     }
//! }
//!
//! ```
//!
//! More examples can be found in the examples folder in the repository
//! and can be run with ```cargo run --example example_name```

/*
    game_wrapper will hold all the code that runs the game
    re-exports: vek, gltf, glium, fastrand
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
