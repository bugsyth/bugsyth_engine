use crate::{context::Context, error::EngineResult};
use glium::{
    Texture2d,
    texture::RawImage2d,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler},
};
use image::ImageReader;
use std::path::Path;

/// Holds texture data
pub struct Texture {
    texture: Texture2d,
}

impl Texture {
    /// Creates a texture from the path to an image
    pub fn new(ctx: &Context, path: impl AsRef<Path>) -> EngineResult<Self> {
        let img = ImageReader::open(path)?.decode()?.to_rgba8();
        let img_dimensions = img.dimensions();
        let img = RawImage2d::from_raw_rgba_reversed(&img.into_raw(), img_dimensions);
        Ok(Self {
            texture: Texture2d::new(&ctx.display, img)?,
        })
    }

    pub fn get_texture(&self) -> &Texture2d {
        &self.texture
    }
    pub fn get_texture_no_filtering(&self) -> Sampler<'_, Texture2d> {
        self.get_texture()
            .sampled()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .minify_filter(MinifySamplerFilter::Nearest)
    }

    pub fn get_texture_mut(&mut self) -> &mut Texture2d {
        &mut self.texture
    }
}
