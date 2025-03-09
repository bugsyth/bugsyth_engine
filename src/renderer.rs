use crate::{context::Context, error::EngineResult};
use drawable::Drawable;
use glium::{uniforms::Uniforms, Frame, Surface};

pub mod drawable;
pub mod fxaa;
pub mod skybox;
pub mod texture;

pub trait Renderer {
    /// Some things will need their rendering prepared before drawing such as skyboxes
    fn draw<D, U>(&mut self, ctx: &mut Context, drawable: &D, uniforms: &U) -> EngineResult
    where
        D: Drawable,
        U: Uniforms;
    fn get_surface(&self) -> &impl Surface;
    fn get_surface_mut(&mut self) -> &mut impl Surface;
    fn clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let surface = self.get_surface_mut();
        surface.clear_color(r, g, b, a);
    }
    fn clear_color_and_depth(&mut self, color: (f32, f32, f32, f32), depth: f32) {
        let surface = self.get_surface_mut();
        surface.clear_color_and_depth(color, depth);
    }
    fn get_dimensions(&self) -> (u32, u32) {
        let surface = self.get_surface();
        surface.get_dimensions()
    }
}

pub struct FrameWrapper {
    frame: Frame,
}

impl FrameWrapper {
    pub fn new(frame: Frame) -> Self {
        Self { frame }
    }
    pub fn finish(self) {
        self.frame.finish().unwrap();
    }
}

impl Renderer for FrameWrapper {
    fn draw<D, U>(&mut self, ctx: &mut Context, drawable: &D, uniforms: &U) -> EngineResult
    where
        D: Drawable,
        U: Uniforms,
    {
        self.frame.draw(
            drawable.get_vbo(),
            drawable.get_ibo(),
            ctx.get_program(drawable.get_program())
                .unwrap_or_else(|| panic!("Unable to find program: {}", drawable.get_program())),
            uniforms,
            &drawable.get_draw_params(),
        )?;
        Ok(())
    }
    fn get_surface(&self) -> &impl Surface {
        &self.frame
    }
    fn get_surface_mut(&mut self) -> &mut impl Surface {
        &mut self.frame
    }
}
