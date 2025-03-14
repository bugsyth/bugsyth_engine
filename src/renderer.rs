use crate::{context::Context, error::EngineResult};
use drawable::Drawable;
use glium::{Frame, Surface, uniforms::Uniforms};

pub mod drawable;
pub mod fxaa;
pub mod skybox;
pub mod texture;

/// Trait that abracts from gliums `Surface` trait, if you want lower level access use `get_surface_mut`
pub trait Renderer {
    /// Some things will need their rendering prepared before drawing such as skyboxes
    fn draw<D, U>(&mut self, ctx: &mut Context, drawable: &D, uniforms: &U) -> EngineResult
    where
        D: Drawable,
        U: Uniforms;

    /// Gets a reference to the glium `Surface` from the `Renderer`
    fn get_surface(&self) -> &impl Surface;
    /// Gets a mutable reference to the glium `Surface` from the `Renderer`
    fn get_surface_mut(&mut self) -> &mut impl Surface;

    /// Clears the screen with the given color
    fn clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let surface = self.get_surface_mut();
        surface.clear_color(r, g, b, a);
    }
    /// Clears the depth with the given depth
    fn clear_depth(&mut self, depth: f32) {
        let surface = self.get_surface_mut();
        surface.clear_depth(depth);
    }
    fn clear_stencil(&mut self, value: i32) {
        let surface = self.get_surface_mut();
        surface.clear_stencil(value);
    }
    /// Clears the color and depth of the screen with the given color and depth
    fn clear_color_and_depth(&mut self, color: (f32, f32, f32, f32), depth: f32) {
        let surface = self.get_surface_mut();
        surface.clear_color_and_depth(color, depth);
    }

    /// Gets the dimensions of the `Renderer`
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
