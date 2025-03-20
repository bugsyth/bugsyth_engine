// Refactor the width and height code into their own functions

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

/// Struct for holding and rendering text, requires a `Font` to be sent to the `Context`,
/// chars that fall below the line they're written on (such as p, q, g, j, etc...)
/// will be part out of the text's height because of their tail.
pub struct Text {
    pub pos: Vec2<f32>,
    width: f32,
    height: f32,
    vbo: VertexBuffer<TextVertex>,
    ibo: IndexBuffer<u16>,
    text: String,
    font_name: String,
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

        let text = text.into();
        let chars = text.chars();
        let mut tallest: f32 = 0.0;

        let font = if let Some(font) = ctx.get_font(&name) {
            font
        } else {
            return Err(EngineError::Error(format!(
                "Font named '{}' not found in the context, it needs to be added first",
                name
            )));
        };

        chars.clone().for_each(|char| {
            if let Some(glyph) = font.glyphs.get(&char) {
                tallest = tallest.max(glyph.height);
            }
        });

        let words = split_to_words(&text);

        let mut vertices = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let mut current_index = 0;
        let mut width: f32 = 0.0;
        let mut height = tallest;
        let mut x = 0.0;
        let mut y = 0.0;
        for (i, word) in words.iter().enumerate() {
            let word_width = get_width(scale, &word, font);
            let next_word_index = i + 1;
            let next_word_width = get_width(
                scale,
                words.get(next_word_index).unwrap_or(&"".to_string()),
                font,
            );
            if let Some((width, spacing)) = wrapping {
                if x + word_width > width || x + next_word_width + word_width > width {
                    x = 0.0;
                    y += spacing;
                    height += spacing
                }
            }

            for char in word.chars() {
                if char.is_whitespace() && x == 0.0 && i != 0 {
                    continue;
                }
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
            height: height * scale,
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

    pub fn get_text_width(
        ctx: &Context,
        scale: f32,
        wrapping: Option<(f32, f32)>,
        text: impl Into<String>,
        font_name: impl Into<String>,
    ) -> EngineResult<f32> {
        let name = font_name.into();

        let text = text.into();
        let chars = text.chars();
        let mut tallest: f32 = 0.0;

        let font = if let Some(font) = ctx.get_font(&name) {
            font
        } else {
            return Err(EngineError::Error(format!(
                "Font named '{}' not found in the context, it needs to be added first",
                name
            )));
        };

        chars.clone().for_each(|char| {
            if let Some(glyph) = font.glyphs.get(&char) {
                tallest = tallest.max(glyph.height);
            }
        });

        let words = split_to_words(&text);

        let mut width: f32 = 0.0;
        let mut x = 0.0;
        for (i, word) in words.iter().enumerate() {
            let word_width = get_width(scale, &word, font);
            let next_word_index = i + 1;
            let next_word_width = get_width(
                scale,
                words.get(next_word_index).unwrap_or(&"".to_string()),
                font,
            );
            if let Some((width, _)) = wrapping {
                if x + word_width > width || x + next_word_width + word_width > width {
                    x = 0.0;
                }
            }

            for char in word.chars() {
                if char.is_whitespace() && x == 0.0 && i != 0 {
                    continue;
                }
                if let Some(glyph) = font.glyphs.get(&char) {
                    x += glyph.x_advance;
                    width = width.max(x * scale);
                }
                if char == ' ' {
                    x += font.font_size / 2.0;
                }
            }
        }

        Ok(width)
    }

    pub fn get_text_height(
        ctx: &Context,
        scale: f32,
        wrapping: Option<(f32, f32)>,
        text: impl Into<String>,
        font_name: impl Into<String>,
    ) -> EngineResult<f32> {
        let name = font_name.into();

        let text = text.into();
        let chars = text.chars();
        let mut tallest: f32 = 0.0;

        let font = if let Some(font) = ctx.get_font(&name) {
            font
        } else {
            return Err(EngineError::Error(format!(
                "Font named '{}' not found in the context, it needs to be added first",
                name
            )));
        };

        chars.clone().for_each(|char| {
            if let Some(glyph) = font.glyphs.get(&char) {
                tallest = tallest.max(glyph.height);
            }
        });

        let words = split_to_words(&text);

        let mut width: f32 = 0.0;
        let mut height = tallest;
        let mut x = 0.0;
        for (i, word) in words.iter().enumerate() {
            let word_width = get_width(scale, &word, font);
            let next_word_index = i + 1;
            let next_word_width = get_width(
                scale,
                words.get(next_word_index).unwrap_or(&"".to_string()),
                font,
            );
            if let Some((width, spacing)) = wrapping {
                if x + word_width > width || x + next_word_width + word_width > width {
                    x = 0.0;
                    height += spacing
                }
            }

            for char in word.chars() {
                if char.is_whitespace() && x == 0.0 && i != 0 {
                    continue;
                }
                if let Some(glyph) = font.glyphs.get(&char) {
                    x += glyph.x_advance;
                    width = width.max(x * scale);
                }
                if char == ' ' {
                    x += font.font_size / 2.0;
                }
            }
        }

        Ok(height * scale)
    }

    pub fn get_text_dimensions(
        ctx: &Context,
        scale: f32,
        wrapping: Option<(f32, f32)>,
        text: impl Into<String>,
        font_name: impl Into<String>,
    ) -> EngineResult<(f32, f32)> {
        let text = text.into();
        let font_name = font_name.into();
        let dimensions = (
            Self::get_text_width(ctx, scale, wrapping, &text, &font_name)?,
            Self::get_text_height(ctx, scale, wrapping, text, font_name)?,
        );

        Ok(dimensions)
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }
    pub fn get_height(&self) -> f32 {
        self.height
    }
    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }
    pub fn get_text(&self) -> &str {
        &self.text
    }
    pub fn get_font(&self) -> &str {
        &self.font_name
    }
}

fn split_to_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let chars: Vec<char> = text.chars().collect();
    for (i, char) in chars.iter().enumerate() {
        if char.is_whitespace() {
            if !current_word.is_empty() && {
                if let Some(next) = chars.get(i + 1) {
                    !next.is_whitespace()
                } else {
                    true
                }
            } {
                words.push(current_word.clone());
                current_word.clear();
            }
            words.push(char.to_string());
        } else {
            current_word.push(*char);
        }
    }
    if !current_word.is_empty() {
        words.push(current_word);
    }
    words
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
