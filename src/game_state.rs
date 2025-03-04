use crate::{context::Context, renderer::Renderer};
use glium::winit::event::WindowEvent;

pub(crate) mod app_wrapper;
mod dt;

#[allow(unused_variables)]
pub trait GameState {
    fn init(&mut self, ctx: &mut Context) {}
    fn update(&mut self, ctx: &mut Context) {}
    /// Runs after update()
    fn fixed_update(&mut self, ctx: &mut Context) {}
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {}
    fn event(&mut self, ctx: &mut Context, event: &WindowEvent) {}
}
