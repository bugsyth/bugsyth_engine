use crate::{error::EngineResult, renderer::texture::Texture};
use glium::{Display, glutin::surface::WindowSurface};
use image::{GrayImage, ImageBuffer, Rgb};
use std::collections::HashMap;

const ATLAS_PADDING: usize = 10;

pub struct Font {
    pub atlas: Texture,
    pub glyphs: HashMap<char, GlyphData>,
    pub font_size: f32,
}

impl Font {
    pub fn new(
        display: &Display<WindowSurface>,
        font_data: &[u8],
        font_size: f32,
    ) -> EngineResult<Self> {
        let font = fontdue::Font::from_bytes(
            font_data,
            fontdue::FontSettings {
                scale: font_size,
                ..Default::default()
            },
        )?;

        let charset: Vec<char> = (32..127).map(|c| c as u8 as char).collect();
        let mut glyphs = HashMap::new();

        let atlas_width = font_size * 24.0;
        let atlas_height = font_size * 8.0;
        let mut atlas = GrayImage::new(atlas_width as u32, atlas_height as u32);

        let mut x_offset: usize = ATLAS_PADDING;
        let mut y_offset: usize = ATLAS_PADDING;
        let mut max_row_height = 0;

        for &c in &charset {
            let (metrics, bitmap) = font.rasterize(c, font_size);
            if metrics.width == 0 || metrics.height == 0 {
                continue;
            }

            if x_offset + metrics.width + ATLAS_PADDING > atlas_width as usize - ATLAS_PADDING {
                x_offset = ATLAS_PADDING;
                y_offset += max_row_height + ATLAS_PADDING;
                max_row_height = 0;
            }

            for row in 0..metrics.height {
                for col in 0..metrics.width {
                    let pixel = bitmap[row * metrics.width + col];
                    atlas.put_pixel(
                        (x_offset + col) as u32,
                        (y_offset + row) as u32,
                        image::Luma([pixel]),
                    );
                }
            }

            let u0 = x_offset as f32 / atlas_width as f32;
            let v0 = 1.0 - (y_offset + metrics.height) as f32 / atlas_height as f32;
            let u1 = (x_offset + metrics.width) as f32 / atlas_width as f32;
            let v1 = 1.0 - y_offset as f32 / atlas_height as f32;

            glyphs.insert(
                c,
                GlyphData {
                    u0,
                    v0,
                    u1,
                    v1,
                    width: metrics.width as f32,
                    height: metrics.height as f32,
                    x_offset: metrics.xmin as f32,
                    y_offset: metrics.ymin as f32,
                    x_advance: metrics.advance_width as f32,
                },
            );

            x_offset += metrics.width + ATLAS_PADDING;
            max_row_height = max_row_height.max(metrics.height);
        }

        let (img_width, img_height) = atlas.dimensions();
        let mut rgb_img = ImageBuffer::new(img_width, img_height);

        for (x, y, pixel) in atlas.enumerate_pixels() {
            let gray = pixel.0[0];
            rgb_img.put_pixel(x, y, Rgb([gray, gray, gray]));
        }
        //rgb_img.save("atlas.png")?;
        let atlas = Texture::from_rgb_bytes(display, &rgb_img.into_raw(), (img_width, img_height))?;

        Ok(Self {
            atlas,
            glyphs,
            font_size,
        })
    }
}

#[derive(Debug)]
pub struct GlyphData {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
    pub width: f32,
    pub height: f32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub x_advance: f32,
}
