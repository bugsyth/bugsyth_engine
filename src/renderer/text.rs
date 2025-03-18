use crate::{
    Context,
    context::font::Font,
    error::{EngineError, EngineResult},
    renderer::Renderer,
};
use glium::{
    DrawParameters, IndexBuffer, Surface, VertexBuffer, implement_vertex, index::PrimitiveType,
    uniform,
};
use vek::Vec2;

pub struct Text {
    pub pos: Vec2<f32>,
    pub width: f32,
    pub height: f32,
    vbo: VertexBuffer<TextVertex>,
    ibo: IndexBuffer<u16>,
    pub text: String,
    pub font_name: String,
}

impl Text {
    /// Creates new text that can be drawn to the screen, wrapping: Option<(width, spacing for next line)>
    pub fn new(
        ctx: &Context,
        pos: Vec2<f32>,
        scale: f32,
        wrapping: Option<(f32, f32)>,
        text: impl Into<String>,
        font_name: impl Into<String>,
    ) -> EngineResult<Self> {
        let name = font_name.into();
        let font = if let Some(font) = ctx.get_font(&name) {
            font
        } else {
            return Err(EngineError::Error(format!(
                "Font named '{}' not found in the context, it needs to be added first",
                name
            )));
        };

        let mut vertices = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let mut current_index = 0;
        let mut x = 0.0;
        let mut y = 0.0;

        let text = text.into();
        let chars = text.chars();
        let mut tallest: f32 = 0.0;

        chars.clone().for_each(|char| {
            if let Some(glyph) = font.glyphs.get(&char) {
                tallest = tallest.max(glyph.height);
            }
        });

        let mut width: f32 = 0.0;
        let mut words = Vec::new();
        let mut current_word = String::new();
        for char in text.chars() {
            if char.is_whitespace() {
                if !current_word.is_empty() {
                    words.push(current_word.clone());
                    current_word.clear();
                }
                words.push(char.to_string());
            } else {
                current_word.push(char);
            }
        }
        if !current_word.is_empty() {
            words.push(current_word);
        }

        for (i, word) in words.iter().enumerate() {
            let word_width = Text::get_width(&ctx, scale, &text, &name)?;
            let next_word_index = i + 1;
            let next_word_width = Text::get_width(
                &ctx,
                scale,
                words.get(next_word_index).unwrap_or(&"".to_string()),
                &name,
            )?;
            if let Some((width, spacing)) = wrapping {
                if x + word_width > width || x + next_word_width + word_width > width {
                    x = 0.0;
                    y += spacing;
                }
            }

            for char in word.chars() {
                if let Some(glyph) = font.glyphs.get(&char) {
                    // Add on y because in OpenGL +y is up
                    let x0 = (x + glyph.x_offset) * scale;
                    let y0 = (-y + -tallest + glyph.y_offset) * scale;
                    let x1 = (x + glyph.width + glyph.x_offset) * scale;
                    let y1 = (-y + glyph.height - tallest + glyph.y_offset) * scale;

                    vertices.push(TextVertex::new([x0, y0], [glyph.u0, glyph.v0]));
                    vertices.push(TextVertex::new([x1, y0], [glyph.u1, glyph.v0]));
                    vertices.push(TextVertex::new([x1, y1], [glyph.u1, glyph.v1]));
                    vertices.push(TextVertex::new([x0, y1], [glyph.u0, glyph.v1]));

                    indices.push(current_index);
                    indices.push(current_index + 1);
                    indices.push(current_index + 2);
                    indices.push(current_index);
                    indices.push(current_index + 2);
                    indices.push(current_index + 3);

                    current_index += 4;
                    x += glyph.x_advance;
                    width = width.max(x * scale);
                }
                if char == ' ' {
                    x += font.font_size / 2.0;
                }
            }
        }

        let vbo = VertexBuffer::new(&ctx.display, &vertices)?;
        let ibo = IndexBuffer::new(&ctx.display, PrimitiveType::TrianglesList, &indices)?;

        Ok(Self {
            pos,
            width,
            height: tallest * scale,
            vbo,
            ibo,
            text,
            font_name: name,
        })
    }

    pub fn draw(&self, ctx: &Context, renderer: &mut impl Renderer) -> EngineResult {
        let program = ctx.get_program("text").unwrap();
        let surface = renderer.get_surface_mut();
        let texture = if let Some(font) = ctx.get_font(&self.font_name) {
            font.atlas.get_texture()
        } else {
            return Err(EngineError::Error(format!(
                "Font '{}' not found",
                self.font_name
            )))?;
        };
        let uniforms = uniform! {
            tex: texture,
            pos: self.pos.into_array(),
        };
        surface.draw(
            &self.vbo,
            &self.ibo,
            program,
            &uniforms,
            &DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            },
        )?;
        Ok(())
    }

    // TODO: add wrapping support for get_width, etc..

    pub fn get_width(
        ctx: &Context,
        scale: f32,
        text: impl Into<String>,
        font_name: impl Into<String>,
    ) -> EngineResult<f32> {
        let text = text.into();
        let name = font_name.into();
        let font = if let Some(font) = ctx.get_font(&name) {
            font
        } else {
            return Err(EngineError::Error(format!(
                "Font named '{}' not found in the context, it needs to be added first",
                name
            )));
        };
        Ok(get_width(scale, &text, font))
    }

    pub fn get_height(
        ctx: &Context,
        scale: f32,
        text: impl Into<String>,
        font_name: impl Into<String>,
    ) -> EngineResult<f32> {
        let text = text.into();
        let name = font_name.into();
        let font = if let Some(font) = ctx.get_font(&name) {
            font
        } else {
            return Err(EngineError::Error(format!(
                "Font named '{}' not found in the context, it needs to be added first",
                name
            )));
        };
        Ok(get_height(scale, &text, font))
    }

    pub fn get_dimensions(
        ctx: &Context,
        scale: f32,
        text: impl Into<String>,
        font_name: impl Into<String>,
    ) -> EngineResult<(f32, f32)> {
        let text = text.into();
        let name = font_name.into();
        let font = if let Some(font) = ctx.get_font(&name) {
            font
        } else {
            return Err(EngineError::Error(format!(
                "Font named '{}' not found in the context, it needs to be added first",
                name
            )));
        };

        let dimensions = (
            get_width(scale, &text, font),
            get_height(scale, &text, font),
        );
        Ok(dimensions)
    }
}

fn get_width(scale: f32, text: &str, font: &Font) -> f32 {
    let mut width = 0.0;
    for char in text.chars() {
        if let Some(glyph) = font.glyphs.get(&char) {
            width += glyph.x_advance;
        }
        if char == ' ' {
            width += font.font_size / 2.0;
        }
    }
    width * scale
}

fn get_height(scale: f32, text: &str, font: &Font) -> f32 {
    let mut tallest: f32 = 0.0;
    for char in text.chars() {
        if let Some(glyph) = font.glyphs.get(&char) {
            tallest = tallest.max(glyph.height);
        }
    }
    tallest * scale
}

#[derive(Clone, Copy)]
struct TextVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(TextVertex, position, tex_coords);

impl TextVertex {
    fn new(position: [f32; 2], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords,
        }
    }
}
