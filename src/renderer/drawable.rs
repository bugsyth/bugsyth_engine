use glium::{DrawParameters, index::IndicesSource, vertex::MultiVerticesSource};

/// Trait for anything that can be passed through the `draw` function
pub trait Drawable {
    fn get_vbo(&self) -> impl MultiVerticesSource;
    fn get_ibo(&self) -> impl Into<IndicesSource>;
    fn get_program(&self) -> String;
    fn get_draw_params(&self) -> DrawParameters;
}
