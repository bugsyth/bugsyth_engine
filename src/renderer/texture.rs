use crate::{context::Context, error::EngineResult};
use glium::texture::{CompressedTexture2d, RawImage2d};
use image::ImageReader;
use std::path::Path;

pub struct Texture {
    texture: CompressedTexture2d,
}

impl Texture {
    pub fn new(ctx: &Context, path: impl AsRef<Path>) -> EngineResult<Self> {
        let img = ImageReader::open(path)?.decode()?.to_rgb8();
        let img_dimensions = img.dimensions();
        let img = RawImage2d::from_raw_rgb_reversed(&img.into_raw(), img_dimensions);
        Ok(Self {
            texture: CompressedTexture2d::new(&ctx.display, img)?,
        })
    }
    pub fn get_texture(&self) -> &CompressedTexture2d {
        &self.texture
    }
}
